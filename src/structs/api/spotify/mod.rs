pub mod album;
pub mod components;
pub mod track;

use crate::{
    macros::if_else,
    statics::{spotify::SPOTIFY_EMBED_COLOR, CONFIG},
    structs::api::oauth::OAuth,
};
use anyhow::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub struct Spotify {
    pub token: String,
}

impl Spotify {
    pub async fn query<T: Display, U: DeserializeOwned>(endpoint: T) -> Result<U> {
        Ok(Client::new()
            .get(format!("https://api.spotify.com/v1/{endpoint}"))
            .header(
                "authorization",
                OAuth::new(
                    "spotify",
                    Client::new()
                        .post("https://accounts.spotify.com/api/token")
                        .header("content-type", "application/x-www-form-urlencoded")
                        .header("authorization", format!("Basic {}", CONFIG.api.spotify_token))
                        .body("grant_type=client_credentials"),
                )
                .get_token()
                .await?,
            )
            .send()
            .await?
            .json::<U>()
            .await?)
    }

    pub fn generate_scannable<T: Display>(uri: T) -> String {
        format!(
            "https://scannables.scdn.co/uri/plain/png/{}/black/700/{uri}",
            SPOTIFY_EMBED_COLOR.chars().skip(1).collect::<String>()
        )
    }

    pub fn format_duration(mut millis: u64) -> String {
        let hours = millis / 3600000;
        millis -= 3600000 * hours;

        let mins = millis / 60000;
        millis -= 60000 * mins;

        let secs = millis / 1000;

        format!(
            "{}{}:{}",
            if_else!(hours > 0, format!("{hours}:"), "".into()),
            if_else!(hours > 0, format!("{mins:02}"), mins.to_string()),
            format!("{secs:02}")
        )
    }
}