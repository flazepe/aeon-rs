use crate::{
    functions::{format_timestamp, label_num, TimestampFormat},
    macros::yes_no,
    structs::api::steam::{
        statics::{STEAM_COUNTRIES, STEAM_EMBED_COLOR, STEAM_USER_STATES},
        user_bans::SteamUserBans,
        Steam,
    },
    traits::Commas,
};
use anyhow::{Context, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize)]
struct SteamUsersResponse {
    response: SteamUsers,
}

#[derive(Deserialize)]
struct SteamUsers {
    players: Vec<SteamUser>,
}

#[derive(Deserialize)] // Can't use `rename_all = "lowercase"` since serde doesn't remove underscores
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

impl SteamUser {
    pub fn format(&self) -> Embed {
        let mut vanity = self.profile_url.clone();

        vanity = vanity.chars().take(vanity.len() - 1).collect::<String>().split('/').last().unwrap_or("None").to_string();

        let mut embed = Embed::new()
            .set_color(STEAM_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(&self.avatar_full)
            .set_title(self.real_name.as_ref().unwrap_or(&self.persona_name))
            .set_url(&self.profile_url)
            .add_field("ID", &self.id, true)
            .add_field(
                "Custom ID",
                match self.id == vanity {
                    true => "None".into(),
                    false => format!("`{vanity}`"),
                },
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
                self.time_created.map_or_else(|| "N/A".into(), |time_created| format_timestamp(time_created, TimestampFormat::Full)),
                false,
            )
            .add_field(
                "Location",
                match STEAM_COUNTRIES.get_key_value(self.loc_country_code.as_deref().unwrap_or("")) {
                    Some((country_code, country)) => format!(
                        ":flag_{}: {}{}",
                        country_code.to_lowercase(),
                        self.loc_state_code.as_ref().map_or_else(
                            || "".into(),
                            |state_code| format!("{}, ", country.states.get(state_code.as_str()).unwrap_or(&"Unknown"))
                        ),
                        country.name,
                    ),
                    None => "N/A".into(),
                },
                true,
            )
            .add_field(
                "Playing",
                self.game_extra_info.as_ref().map_or_else(
                    || "None".into(),
                    |game_extra_info| {
                        format!(
                            "[{}](https://store.steampowered.com/app/{}){}",
                            game_extra_info,
                            self.game_id.unwrap_or(0),
                            format!("\n{}", self.game_server_ip.as_deref().unwrap_or("")).trim(),
                        )
                    },
                ),
                true,
            )
            .add_field("Allows Profile Comments", yes_no!(self.comment_permission.is_some()), true);

        if let Some(bans) = self.bans.as_ref() {
            embed = embed
                .add_field("Community Banned", yes_no!(bans.community_banned), true)
                .add_field(
                    "VAC Banned",
                    format!(
                        "{} ({}, {})",
                        yes_no!(bans.vac_banned),
                        label_num(bans.vac_bans, "VAC ban", "VAC bans"),
                        label_num(bans.game_bans, "game ban", "game bans"),
                    ),
                    true,
                )
                .add_field("Days Since Last Ban", bans.days_since_last_ban.commas(), true);
        }

        embed
    }
}

impl Steam {
    pub async fn get_user<T: Display>(id: T) -> Result<SteamUser> {
        let mut id = id.to_string();

        if !id.chars().all(|char| char.is_numeric()) {
            id = Steam::get_user_vanity(&id).await?;
        }

        let mut user = Steam::query::<_, _, SteamUsersResponse>("GetPlayerSummaries/v0002/", &[("steamids", id.as_str())])
            .await?
            .response
            .players
            .into_iter()
            .next()
            .context("User not found.")?;

        // Get user bans
        user.bans = Steam::get_user_bans(&user.id).await.ok();

        Ok(user)
    }
}
