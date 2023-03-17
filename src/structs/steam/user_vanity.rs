use anyhow::{Context, Result};
use reqwest::get;
use serde::Deserialize;

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
    pub async fn get(player: &str, api_key: &str) -> Result<String> {
        Ok(get(format!(
			"http://api.steampowered.com/ISteamUser/ResolveVanityURL/v0001/?key={api_key}&vanityurl={player}"
		))
        .await?
        .json::<ResolveVanityURLEndpoint>()
        .await?
        .response
        .id
        .context("User not found.")?)
    }
}
