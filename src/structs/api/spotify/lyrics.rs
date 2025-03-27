use crate::{
    functions::limit_strings,
    statics::{CACHE, CONFIG, REQWEST},
    structs::api::{
        google::{Google, statics::GOOGLE_TRANSLATE_LANGUAGES},
        spotify::{
            Spotify,
            statics::{SPOTIFY_EMBED_AUTHOR_ICON_URL, SPOTIFY_EMBED_AUTHOR_URL, SPOTIFY_EMBED_COLOR},
            track::SpotifyFullTrack,
        },
    },
};
use anyhow::{Context, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::{
    fmt::Display,
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};
use totp_rs::{Algorithm, Secret, TOTP};

static SPOTIFY_TOTP: LazyLock<TOTP> = LazyLock::new(|| {
    let secret = generate_totp_secret([12, 56, 76, 33, 88, 44, 88, 33, 78, 78, 11, 66, 22, 22, 55, 69, 54]).unwrap();
    TOTP::new(Algorithm::SHA1, 6, 1, 30, secret).unwrap()
});

fn generate_totp_secret(secret: [usize; 17]) -> Result<Vec<u8>> {
    let transformed = secret.iter().enumerate().fold(String::new(), |acc, (index, entry)| acc + &(entry ^ ((index % 33) + 9)).to_string());
    Ok(Secret::Raw(transformed.as_bytes().to_vec()).to_bytes()?)
}

#[derive(Deserialize, Debug)]
pub struct SpotifyLyricsWithTrack {
    pub track: SpotifyFullTrack,
    pub lyrics: Vec<String>,
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
#[allow(dead_code)]
struct SpotifyAccessToken {
    access_token: String,
    access_token_expiration_timestamp_ms: u128,
}

impl SpotifyLyricsWithTrack {
    pub fn format(&self) -> Embed {
        let thumbnail = self.track.album.images.first().map_or("", |image| image.url.as_str());
        let title = format!("{} - {}", self.track.artists[0].name, self.track.name).chars().take(256).collect::<String>();
        let url = &self.track.external_urls.spotify;
        let description = limit_strings(&self.lyrics, "\n", 4096);

        Embed::new()
            .set_color(SPOTIFY_EMBED_COLOR)
            .unwrap_or_default()
            .set_thumbnail(thumbnail)
            .set_author("Spotify  •  Lyrics", Some(SPOTIFY_EMBED_AUTHOR_URL), Some(SPOTIFY_EMBED_AUTHOR_ICON_URL))
            .set_title(title)
            .set_url(url)
            .set_description(description)
    }
}

impl Spotify {
    pub async fn get_access_token() -> Result<String> {
        {
            let access_token = CACHE.spotify_access_token.read().unwrap();

            if !access_token.0.is_empty() && access_token.1 > SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() {
                return Ok(access_token.0.clone());
            }
        }

        let cookie = format!("sp_dc={}", CONFIG.api.spotify_dc);

        let res = REQWEST
            .get("https://open.spotify.com/get_access_token")
            .query(&[("productType", "web-player"), ("totp", &SPOTIFY_TOTP.generate_current()?), ("totpVer", "5")])
            .header("user-agent", "yes")
            .header("cookie", cookie)
            .send()
            .await?
            .json::<SpotifyAccessToken>()
            .await
            .context("Could not get user access token.")?;

        let mut access_token = CACHE.spotify_access_token.write().unwrap();

        access_token.0 = res.access_token.clone();
        access_token.1 = res.access_token_expiration_timestamp_ms;

        Ok(res.access_token)
    }

    pub async fn get_lyrics<T: Display>(track: SpotifyFullTrack, translate_language: Option<T>) -> Result<SpotifyLyricsWithTrack> {
        let url = format!("https://spclient.wg.spotify.com/color-lyrics/v2/track/{}?format=json", track.id);
        let access_token = Self::get_access_token().await?;
        let mut lyrics = REQWEST
            .get(url)
            .bearer_auth(access_token)
            .header("user-agent", "yes")
            .header("app-platform", "WebPlayer")
            .send()
            .await?
            .json::<SpotifyLyrics>()
            .await
            .context("Could not get song lyrics.")?
            .lyrics
            .lines
            .iter()
            .map(|line| line.words.replace('♪', ""))
            .collect::<Vec<String>>();

        // Translate lines
        let translate_language = translate_language.map(|translate_language| translate_language.to_string()).unwrap_or_default();

        if GOOGLE_TRANSLATE_LANGUAGES.contains_key(translate_language.as_str()) {
            let translated = Google::translate(lyrics.join("\n"), "auto", &translate_language).await?.translation;
            let mut translated_lines = translated.split('\n');

            for line in lyrics.iter_mut() {
                let Some(translated_line) = translated_lines.next() else { break };

                if !translated_line.is_empty() {
                    line.push_str(&format!("\n-# {translated_line}"));
                }
            }
        }

        Ok(SpotifyLyricsWithTrack { track, lyrics })
    }
}
