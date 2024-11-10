use crate::structs::api::steam::Steam;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
struct SteamUserVanityResponse {
    response: SteamUserVanity,
}

#[derive(Deserialize, Debug)]
pub struct SteamUserVanity {
    #[serde(rename = "steamid")]
    pub id: Option<String>,
    pub success: u64,
    pub message: Option<String>,
}

impl Steam {
    pub async fn get_user_vanity<T: Display>(player: T) -> Result<String> {
        Self::query::<_, _, SteamUserVanityResponse>("ResolveVanityURL/v0001/", &[("vanityurl", player.to_string().as_str())])
            .await?
            .response
            .id
            .context("Invalid user vanity.")
    }
}
