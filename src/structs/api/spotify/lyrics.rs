use crate::{
    statics::{colors::PRIMARY_COLOR, CONFIG, REQWEST},
    structs::api::spotify::{track::SpotifyFullTrack, Spotify},
};
use anyhow::{bail, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;

#[derive(Deserialize)]
pub struct SpotifyLyricsWithTrack {
    pub track: SpotifyFullTrack,
    pub lyrics: SpotifyLyrics,
}

#[derive(Deserialize)]
pub struct SpotifyLyrics {
    pub lyrics: SpotifyRawLyrics,
}

#[derive(Deserialize)]
pub struct SpotifyRawLyrics {
    pub lines: Vec<SpotifyLyricsLine>,
}

#[derive(Deserialize)]
pub struct SpotifyLyricsLine {
    pub words: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotifyToken {
    pub access_token: String,
    pub access_token_expiration_timestamp_ms: i64,
}

impl SpotifyLyricsWithTrack {
    pub fn format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(self.track.album.images.first().map_or(&"".into(), |image: &super::components::SpotifyImage| &image.url))
            .set_author(
                self.track.artists[0].name.chars().take(256).collect::<String>(),
                Some(&self.track.artists[0].external_urls.spotify),
                None::<String>,
            )
            .set_title(self.track.name.chars().take(256).collect::<String>())
            .set_url(&self.track.external_urls.spotify)
            .set_description(
                self.lyrics
                    .lyrics
                    .lines
                    .iter()
                    .map(|line| line.words.clone().replace('♪', ""))
                    .collect::<Vec<String>>()
                    .join("\n")
                    .chars()
                    .take(4096)
                    .collect::<String>(),
            )
    }
}

impl Spotify {
    pub async fn get_user_token() -> Result<SpotifyToken> {
        match REQWEST
            .get("https://open.spotify.com/get_access_token?reason=transport&productType=web_player")
            .header("user-agent", "yes")
            .header("cookie", format!("sp_dc={}", &CONFIG.api.spotify_dc))
            .send()
            .await?
            .json::<SpotifyToken>()
            .await
        {
            Ok(token) => Ok(token),
            Err(_) => bail!("Could not get user token."),
        }
    }

    pub async fn get_lyrics(track: SpotifyFullTrack) -> Result<SpotifyLyricsWithTrack> {
        match REQWEST
            .get(format!("https://spclient.wg.spotify.com/color-lyrics/v2/track/{}?format=json", track.id))
            .bearer_auth(Spotify::get_user_token().await?.access_token)
            .header("app-platform", "WebPlayer")
            .send()
            .await?
            .json::<SpotifyLyrics>()
            .await
        {
            Ok(lyrics) => Ok(SpotifyLyricsWithTrack { track, lyrics }),
            Err(_) => bail!("Could not get song lyrics."),
        }
    }
}
