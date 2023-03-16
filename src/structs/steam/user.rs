use crate::{
    constants::STEAM_USER_STATES,
    format_timestamp,
    structs::steam::{countries::*, user_bans::*, user_vanity::*},
};
use anyhow::{Context, Result};
use reqwest::get;
use serde::Deserialize;
use slashook::structs::embeds::Embed;

#[derive(Deserialize)]
pub struct SteamUser {
    #[serde(rename = "steamid")]
    id: String,

    #[serde(rename = "communityvisibilitystate")]
    community_visibility_state: u64,

    #[serde(rename = "profilestate")]
    profile_state: u64,

    #[serde(rename = "personaname")]
    persona_name: String,

    #[serde(rename = "commentpermission")]
    comment_permission: Option<u64>,

    #[serde(rename = "profileurl")]
    profile_url: String,

    avatar: String,

    #[serde(rename = "avatarmedium")]
    avatar_medium: String,

    #[serde(rename = "avatarfull")]
    avatar_full: String,

    #[serde(rename = "avatarhash")]
    avatar_hash: String,

    #[serde(rename = "lastlogoff")]
    last_log_off: Option<u64>,

    #[serde(rename = "personastate")]
    persona_state: u64,

    #[serde(rename = "realname")]
    real_name: Option<String>,

    #[serde(rename = "primaryclanid")]
    primary_clan_id: String,

    #[serde(rename = "timecreated")]
    time_created: u64,

    #[serde(rename = "personastateflags")]
    persona_state_flags: u64,

    #[serde(rename = "loccountrycode")]
    loc_country_code: Option<String>,

    #[serde(rename = "locstatecode")]
    loc_state_code: Option<String>,

    #[serde(rename = "loccityid")]
    loc_city_id: Option<u64>,

    #[serde(rename = "gameextrainfo")]
    game_extra_info: Option<String>,

    #[serde(rename = "gameid")]
    game_id: Option<u64>,

    #[serde(rename = "gameserverip")]
    game_server_ip: Option<String>,

    // We have to fetch this from another endpoint
    bans: Option<SteamUserBans>,
}

#[derive(Deserialize)]
pub struct SteamUsers {
    players: Vec<SteamUser>,
}

#[derive(Deserialize)]
struct GetPlayerSummariesEndpoint {
    response: SteamUsers,
}

impl SteamUser {
    pub async fn get(user: &str, api_key: &str) -> Result<Self> {
        let mut user = get(format!(
            "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={api_key}&steamids={}",
            SteamUserVanity::get(user, api_key).await?
        ))
        .await?
        .json::<GetPlayerSummariesEndpoint>()
        .await?
        .response
        .players
        .into_iter()
        .next()
        .context("User not found.")?;

        // Get user bans
        user.bans = SteamUserBans::get(&user.id, api_key).await.ok();

        Ok(user)
    }

    pub fn format(self) -> Embed {
        let mut vanity = self.profile_url.clone();

        vanity = vanity
            .chars()
            .take(vanity.len() - 1)
            .collect::<String>()
            .split("/")
            .last()
            .unwrap_or("None")
            .to_string();

        let mut embed = Embed::new()
            .set_thumbnail(self.avatar_full)
            .set_title(self.real_name.unwrap_or(self.persona_name))
            .set_url(self.profile_url)
            .add_field("ID", &self.id, true)
            .add_field(
                "Custom ID",
                if self.id == vanity {
                    "None".into()
                } else {
                    format!("`{vanity}`")
                },
                true,
            )
            .add_field(
                "Status",
                STEAM_USER_STATES
                    .iter()
                    .enumerate()
                    .find(|(index, _)| &(self.persona_state as usize) == index)
                    .and_then(|state| Some(state.1))
                    .unwrap_or(&"Unknown"),
                true,
            )
            .add_field("Created", format_timestamp!(self.time_created), false)
            .add_field(
                "Location",
                if let Some(country) = self.loc_country_code {
                    let country = SteamCountries::init().get(&country);

                    if let Some(country) = country {
                        format!(
                            ":flag_{}: {}{}",
                            country.code.to_lowercase(),
                            self.loc_state_code
                                .and_then(|state_code| Some(format!(
                                    "{}, ",
                                    country
                                        .states
                                        .iter()
                                        .find(|[state, _]| state == &state_code)
                                        .and_then(|state| Some(state[1].as_str()))
                                        .unwrap_or("")
                                )))
                                .unwrap_or("".into()),
                            country.name
                        )
                    } else {
                        "".into()
                    }
                } else {
                    "N/A".into()
                },
                true,
            )
            .add_field(
                "Playing",
                if let Some(game_extra_info) = self.game_extra_info {
                    format!(
                        "[{}](https://store.steampowered.com/app/{}){}",
                        game_extra_info,
                        self.game_id.unwrap_or(0),
                        self.game_server_ip
                            .and_then(|game_server_ip| Some(format!("\n{game_server_ip}")))
                            .unwrap_or("".into())
                    )
                } else {
                    "None".into()
                },
                true,
            )
            .add_field(
                "Allows Profile Comments",
                if self.comment_permission.is_some() {
                    "Yes"
                } else {
                    "No"
                },
                true,
            );

        if let Some(bans) = self.bans {
            embed = embed
                .add_field(
                    "Community Banned",
                    if bans.community_banned { "Yes" } else { "No" },
                    true,
                )
                .add_field(
                    "Vac Banned",
                    format!(
                        "{} ({} VAC ban{}, {} game ban{})",
                        if bans.vac_banned { "Yes" } else { "No" },
                        bans.vac_bans,
                        if bans.vac_bans == 1 { "s" } else { "" },
                        bans.game_bans,
                        if bans.game_bans == 1 { "s" } else { "" }
                    ),
                    true,
                )
                .add_field("Days Since Last Ban", bans.days_since_last_ban, true);
        }

        embed
    }
}
