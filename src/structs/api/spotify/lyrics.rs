use crate::{
    functions::limit_strings,
    statics::{colors::PRIMARY_COLOR, CONFIG, REQWEST},
    structs::api::spotify::{track::SpotifyFullTrack, Spotify},
};
use anyhow::{Context, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;

#[derive(Deserialize, Debug)]
pub struct SpotifyLyricsWithTrack {
    pub track: SpotifyFullTrack,
    pub lyrics: SpotifyLyrics,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyLyrics {
    pub lyrics: SpotifyRawLyrics,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyRawLyrics {
    pub lines: Vec<SpotifyRawLyricsLine>,
}

#[derive(Deserialize, Debug)]
pub struct SpotifyRawLyricsLine {
    pub words: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotifyToken {
    pub access_token: String,
    pub access_token_expiration_timestamp_ms: i64,
}

impl SpotifyLyricsWithTrack {
    pub fn format(&self) -> Embed {
        let thumbnail = self.track.album.images.first().map_or("", |image| image.url.as_str());
        let author_name = self.track.artists[0].name.chars().take(256).collect::<String>();
        let author_url = Some(&self.track.artists[0].external_urls.spotify);
        let title = self.track.name.chars().take(256).collect::<String>();
        let url = &self.track.external_urls.spotify;
        let lyrics = limit_strings(self.lyrics.lyrics.lines.iter().map(|line| line.words.clone().replace('♪', "")), "\n", 4096);

        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(thumbnail)
            .set_author(author_name, author_url, None::<String>)
            .set_title(title)
            .set_url(url)
            .set_description(lyrics)
    }
}

impl Spotify {
    pub async fn get_user_token() -> Result<SpotifyToken> {
        let cookie = format!("sp_dc={}", &CONFIG.api.spotify_dc);

        REQWEST
            .get("https://open.spotify.com/get_access_token?reason=transport&productType=web_player")
            .header("user-agent", "yes")
            .header("cookie", cookie)
            .send()
            .await?
            .json::<SpotifyToken>()
            .await
            .context("Could not get user token.")
    }

    pub async fn get_lyrics(track: SpotifyFullTrack) -> Result<SpotifyLyricsWithTrack> {
        let url = format!("https://spclient.wg.spotify.com/color-lyrics/v2/track/{}?format=json", track.id);
        let bearer_token = Spotify::get_user_token().await?.access_token;

        REQWEST
            .get(url)
            .bearer_auth(bearer_token)
            .header("app-platform", "WebPlayer")
            .send()
            .await?
            .json::<SpotifyLyrics>()
            .await
            .map(|lyrics| SpotifyLyricsWithTrack { track, lyrics })
            .context("Could not get song lyrics.")
    }
}
