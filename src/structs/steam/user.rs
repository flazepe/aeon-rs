use crate::{
    and_then_or,
    constants::STEAM_USER_STATES,
    format_timestamp, if_else, plural,
    structs::steam::{countries::*, user_bans::*, user_vanity::*},
    yes_no,
};
use anyhow::{Context, Result};
use reqwest::get;
use serde::Deserialize;
use slashook::structs::embeds::Embed;

#[derive(Deserialize)]
pub struct SteamUser {
    #[serde(rename = "steamid")]
    pub id: String,

    #[serde(rename = "communityvisibilitystate")]
    pub community_visibility_state: u64,

    #[serde(rename = "profilestate")]
    pub profile_state: u64,

    #[serde(rename = "personaname")]
    pub persona_name: String,

    #[serde(rename = "commentpermission")]
    pub comment_permission: Option<u64>,

    #[serde(rename = "profileurl")]
    pub profile_url: String,

    pub avatar: String,

    #[serde(rename = "avatarmedium")]
    pub avatar_medium: String,

    #[serde(rename = "avatarfull")]
    pub avatar_full: String,

    #[serde(rename = "avatarhash")]
    pub avatar_hash: String,

    #[serde(rename = "lastlogoff")]
    pub last_log_off: Option<u64>,

    #[serde(rename = "personastate")]
    pub persona_state: u64,

    #[serde(rename = "realname")]
    pub real_name: Option<String>,

    #[serde(rename = "primaryclanid")]
    pub primary_clan_id: String,

    #[serde(rename = "timecreated")]
    pub time_created: u64,

    #[serde(rename = "personastateflags")]
    pub persona_state_flags: u64,

    #[serde(rename = "loccountrycode")]
    pub loc_country_code: Option<String>,

    #[serde(rename = "locstatecode")]
    pub loc_state_code: Option<String>,

    #[serde(rename = "loccityid")]
    pub loc_city_id: Option<u64>,

    #[serde(rename = "gameextrainfo")]
    pub game_extra_info: Option<String>,

    #[serde(rename = "gameid")]
    pub game_id: Option<u64>,

    #[serde(rename = "gameserverip")]
    pub game_server_ip: Option<String>,

    // We have to fetch this from another endpoint
    pub bans: Option<SteamUserBans>,
}

#[derive(Deserialize)]
struct SteamUsers {
    players: Vec<SteamUser>,
}

#[derive(Deserialize)]
struct GetPlayerSummariesEndpoint {
    response: SteamUsers,
}

impl SteamUser {
    pub async fn get(user: &str, api_key: &str) -> Result<Self> {
        let mut user = user.to_string();

        if !user.chars().into_iter().all(|char| char.is_numeric()) {
            user = SteamUserVanity::get(&user, api_key).await?;
        }

        let mut user = get(format!(
            "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={api_key}&steamids={user}" 
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
                if_else!(self.id == vanity, "None".into(), format!("`{vanity}`")),
                true,
            )
            .add_field(
                "Status",
                and_then_or!(
                    STEAM_USER_STATES
                        .iter()
                        .enumerate()
                        .find(|(index, _)| &(self.persona_state as usize) == index),
                    |state| Some(state.1),
                    &"Unknown"
                ),
                true,
            )
            .add_field("Created", format_timestamp!(self.time_created), false)
            .add_field(
                "Location",
                match SteamCountries::init().get(&self.loc_country_code.unwrap_or("".into())) {
                    Some(country) => format!(
                        ":flag_{}: {}{}",
                        country.code.to_lowercase(),
                        and_then_or!(
                            self.loc_state_code,
                            |state_code| Some(format!(
                                "{}, ",
                                and_then_or!(
                                    country
                                        .states
                                        .iter()
                                        .find(|[state, _]| state == &state_code),
                                    |state| Some(state[1]),
                                    ""
                                )
                            )),
                            "".into()
                        ),
                        country.name
                    ),
                    None => "N/A".into(),
                },
                true,
            )
            .add_field(
                "Playing",
                and_then_or!(
                    self.game_extra_info,
                    |game_extra_info| {
                        Some(format!(
                            "[{}](https://store.steampowered.com/app/{}){}",
                            game_extra_info,
                            self.game_id.unwrap_or(0),
                            format!("\n{}", self.game_server_ip.unwrap_or("".into())).trim()
                        ))
                    },
                    "None".into()
                ),
                true,
            )
            .add_field(
                "Allows Profile Comments",
                yes_no!(self.comment_permission.is_some()),
                true,
            );

        if let Some(bans) = self.bans {
            embed = embed
                .add_field("Community Banned", yes_no!(bans.community_banned), true)
                .add_field(
                    "Vac Banned",
                    format!(
                        "{} ({}, {})",
                        yes_no!(bans.vac_banned),
                        plural!(bans.vac_bans, "VAC ban"),
                        plural!(bans.game_bans, "game ban")
                    ),
                    true,
                )
                .add_field("Days Since Last Ban", bans.days_since_last_ban, true);
        }

        embed
    }
}
