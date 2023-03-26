use crate::statics::CONFIG;
use anyhow::{Context, Result};
use reqwest::get;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct SteamUserVanity {
    #[serde(rename = "steamid")]
    pub id: Option<String>,
    pub success: u64,
    pub message: Option<String>,
}

#[derive(Deserialize)]
struct ResolveVanityURLEndpoint {
    response: SteamUserVanity,
}

impl SteamUserVanity {
    pub async fn get<T: Display>(player: T) -> Result<String> {
        Ok(get(format!(
            "http://api.steampowered.com/ISteamUser/ResolveVanityURL/v0001/?key={}&vanityurl={player}",
            CONFIG.api.steam_key
        ))
        .await?
        .json::<ResolveVanityURLEndpoint>()
        .await?
        .response
        .id
        .context("Invalid user vanity.")?)
    }
}
