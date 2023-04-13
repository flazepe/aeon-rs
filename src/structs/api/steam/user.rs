use crate::{
    functions::{format_timestamp, TimestampFormat},
    macros::{if_else, plural, yes_no},
    statics::{
        steam::{STEAM_API_ENDPOINT, STEAM_COUNTRIES, STEAM_EMBED_COLOR, STEAM_USER_STATES},
        CONFIG,
    },
    structs::api::steam::{user_bans::SteamUserBans, user_vanity::SteamUserVanity},
};
use anyhow::{Context, Result};
use reqwest::get;
use serde::Deserialize;
use slashook::structs::embeds::Embed;

#[derive(Deserialize)] // Can't use `rename_all = "lowercase"` because it doesn't remove underscores
#[serde(rename_all = "lowercase")]
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
    pub primary_clan_id: Option<String>,

    #[serde(rename = "timecreated")]
    pub time_created: Option<u64>,

    #[serde(rename = "personastateflags")]
    pub persona_state_flags: Option<u64>,

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
struct SteamResponse<T> {
    response: T,
}

impl SteamUser {
    pub async fn get<T: ToString>(id: T) -> Result<Self> {
        let mut id = id.to_string();

        if !id.chars().into_iter().all(|char| char.is_numeric()) {
            id = SteamUserVanity::get(&id).await?;
        }

        let mut user = get(format!(
            "{STEAM_API_ENDPOINT}/GetPlayerSummaries/v0002/?key={}&steamids={id}",
            CONFIG.api.steam_key
        ))
        .await?
        .json::<SteamResponse<SteamUsers>>()
        .await?
        .response
        .players
        .into_iter()
        .next()
        .context("User not found.")?;

        // Get user bans
        user.bans = SteamUserBans::get(&user.id).await.ok();

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
            .set_color(STEAM_EMBED_COLOR)
            .unwrap_or_default()
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
                STEAM_USER_STATES
                    .iter()
                    .enumerate()
                    .find(|(index, _)| &(self.persona_state as usize) == index)
                    .map_or(&"Unknown", |state| state.1),
                true,
            )
            .add_field(
                "Created",
                self.time_created.map_or("N/A".into(), |time_created| {
                    format_timestamp(time_created, TimestampFormat::Full)
                }),
                false,
            )
            .add_field(
                "Location",
                match STEAM_COUNTRIES.get_key_value(self.loc_country_code.unwrap_or("".into()).as_str()) {
                    Some((country_code, country)) => format!(
                        ":flag_{}: {}{}",
                        country_code.to_lowercase(),
                        self.loc_state_code.map_or("".into(), |state_code| format!(
                            "{}, ",
                            country.states.get(state_code.as_str()).unwrap_or(&"Unknown"),
                        )),
                        country.name
                    ),
                    None => "N/A".into(),
                },
                true,
            )
            .add_field(
                "Playing",
                self.game_extra_info.map_or("None".into(), |game_extra_info| {
                    format!(
                        "[{}](https://store.steampowered.com/app/{}){}",
                        game_extra_info,
                        self.game_id.unwrap_or(0),
                        format!("\n{}", self.game_server_ip.unwrap_or("".into())).trim()
                    )
                }),
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
