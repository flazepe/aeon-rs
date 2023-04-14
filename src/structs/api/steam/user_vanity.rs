use crate::structs::api::steam::Steam;
use anyhow::{Context, Result};
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
struct SteamUserVanityResponse {
    response: SteamUserVanity,
}

impl Steam {
    pub async fn get_user_vanity<T: Display>(player: T) -> Result<String> {
        Ok(
            Steam::query::<_, _, SteamUserVanityResponse>("ResolveVanityURL/v0001/", format!("vanityurl={player}"))
                .await?
                .response
                .id
                .context("Invalid user vanity.")?,
        )
    }
}
