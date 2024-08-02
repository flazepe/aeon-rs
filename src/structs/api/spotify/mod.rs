mod album;
mod components;
mod lyrics;
pub mod statics;
mod track;

use crate::{
    statics::{CONFIG, REQWEST},
    structs::api::spotify::statics::SPOTIFY_EMBED_COLOR,
    structs::database::oauth::Oauth,
};
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub struct Spotify;

impl Spotify {
    pub async fn query<T: Display, U: DeserializeOwned>(endpoint: T) -> Result<U> {
        Ok(REQWEST
            .get(format!("https://api.spotify.com/v1/{endpoint}"))
            .header(
                "authorization",
                Oauth::new(
                    "spotify",
                    REQWEST
                        .post("https://accounts.spotify.com/api/token")
                        .header("authorization", format!("Basic {}", CONFIG.api.spotify_token))
                        .form(&[("grant_type", "client_credentials")]),
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
        format!("https://scannables.scdn.co/uri/plain/png/{}/black/700/{uri}", SPOTIFY_EMBED_COLOR.trim_start_matches('#'))
    }

    pub fn format_duration(mut millis: u64) -> String {
        let hours = millis / 3600000;
        millis -= 3600000 * hours;

        let mins = millis / 60000;
        millis -= 60000 * mins;

        let secs = millis / 1000;

        format!(
            "{}:{secs:02}",
            match hours > 0 {
                true => format!("{hours}:{mins:02}"),
                false => mins.to_string(),
            },
        )
    }
}
