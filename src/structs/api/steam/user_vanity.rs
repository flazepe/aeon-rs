use crate::structs::api::steam::Steam;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct SteamUserVanityResponse {
    response: SteamUserVanity,
}

#[derive(Deserialize)]
pub struct SteamUserVanity {
    #[serde(rename = "steamid")]
    pub id: Option<String>,
    pub success: u64,
    pub message: Option<String>,
}

impl Steam {
    pub async fn get_user_vanity<T: ToString>(player: T) -> Result<String> {
        Steam::query::<_, _, SteamUserVanityResponse>("ResolveVanityURL/v0001/", &[("vanityurl", player.to_string().as_str())])
            .await?
            .response
            .id
            .context("Invalid user vanity.")
    }
}
