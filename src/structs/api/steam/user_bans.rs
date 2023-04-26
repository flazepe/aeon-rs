use crate::structs::api::steam::Steam;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct SteamUserBansResponse {
    players: Vec<SteamUserBans>,
}

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

impl Steam {
    pub async fn get_user_bans<T: ToString>(id: T) -> Result<SteamUserBans> {
        Ok(Steam::query::<_, _, SteamUserBansResponse>("GetPlayerBans/v1/", &[("steamids", id.to_string().as_str())])
            .await?
            .players
            .into_iter()
            .next()
            .context("User not found.")?)
    }
}
