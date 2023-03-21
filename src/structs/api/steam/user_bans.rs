use crate::constants::*;
use anyhow::{Context, Result};
use reqwest::get;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SteamUserBans {
    #[serde(rename = "SteamId")]
    pub id: String,

    #[serde(rename = "CommunityBanned")]
    pub community_banned: bool,

    #[serde(rename = "VACBanned")]
    pub vac_banned: bool,

    #[serde(rename = "NumberOfVACBans")]
    pub vac_bans: u64,

    #[serde(rename = "DaysSinceLastBan")]
    pub days_since_last_ban: u64,

    #[serde(rename = "NumberOfGameBans")]
    pub game_bans: u64,

    #[serde(rename = "EconomyBan")]
    pub economy_ban: String,
}

#[derive(Deserialize)]
struct GetPlayerBansEndpoint {
    players: Vec<SteamUserBans>,
}

impl SteamUserBans {
    pub async fn get(id: &str) -> Result<Self> {
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
        .context("user not found")?)
    }
}
