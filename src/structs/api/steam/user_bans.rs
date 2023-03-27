use crate::statics::CONFIG;
use anyhow::{Context, Result};
use reqwest::get;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SteamUserBans {
    #[serde(rename = "SteamId")]
    pub id: String,

    #[serde(rename = "VACBanned")]
    pub vac_banned: bool,

    #[serde(rename = "NumberOfVACBans")]
    pub vac_bans: u64,

    #[serde(rename = "NumberOfGameBans")]
    pub game_bans: u64,

    pub days_since_last_ban: u64,
    pub economy_ban: String,
    pub community_banned: bool,
}

#[derive(Deserialize)]
struct GetPlayerBansEndpoint {
    players: Vec<SteamUserBans>,
}

impl SteamUserBans {
    pub async fn get<T: Display>(id: T) -> Result<Self> {
        Ok(get(format!(
            "http://api.steampowered.com/ISteamUser/GetPlayerBans/v1/?key={}&steamids={id}",
            CONFIG.api.steam_key
        ))
        .await?
        .json::<GetPlayerBansEndpoint>()
        .await?
        .players
        .into_iter()
        .next()
        .context("User not found.")?)
    }
}
