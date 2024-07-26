use crate::statics::{colors::PRIMARY_COLOR, REQWEST};
use anyhow::{bail, Result};

use nipper::Document;
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
pub struct LyricFindSearchResult {
    pub tracks: Vec<LyricFindSearchResultTrack>,
}

#[derive(Deserialize, Debug)]
pub struct LyricFindSearchResultTrack {
    pub title: String,

    #[serde(rename = "camelCase")]
    pub title_simple: Option<String>,

    pub artist: LyricFindSearchResultArtist,
    pub album: Option<LyricFindSearchResultAlbum>,
    pub duration: Option<String>,
    pub language: Option<String>,
    pub available_translations: Option<Vec<String>>,

    pub apple: Option<i64>,
    pub deezer: Option<i64>,
    pub spotify: Option<String>,
    pub isrcs: Option<Vec<String>>,
    pub lrc_verified: Option<bool>,

    pub instrumental: bool,
    pub viewable: bool,

    pub has_lrc: bool,
    pub has_contentfilter: bool,
    pub has_emotion: bool,
    pub has_sentiment: bool,

    pub snippet: String,
    pub context: Option<String>,
    pub last_update: String,
    pub score: f64,
    pub slug: String,
}

#[derive(Deserialize, Debug)]
pub struct LyricFindSearchResultArtist {
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LyricFindSearchResultAlbum {
    pub title: String,
    pub release_year: i64,
    pub cover_art: Option<String>,
    pub slug: String,
}

pub struct LyricFind;

impl LyricFind {
    pub async fn search<T: Display>(query: T) -> Result<Vec<LyricFindSearchResultTrack>> {
        let tracks = REQWEST
            .get("https://lyrics.lyricfind.com/api/v1/search")
            .header("user-agent", "yes")
            .query(&[
                ("reqtype", "default"),
                ("output", "json"),
                ("territory", "US"),
                ("searchtype", "track"),
                ("limit", "25"),
                ("all", &query.to_string()),
            ])
            .send()
            .await?
            .json::<LyricFindSearchResult>()
            .await?
            .tracks;

        if tracks.is_empty() {
            bail!("Song not found.");
        }

        Ok(tracks)
    }
}

impl LyricFindSearchResultTrack {
    pub fn format(&self) -> Embed {
        let mut embed = Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(
                self.album
                    .as_ref()
                    .and_then(|album| album.cover_art.as_ref())
                    .map(|cover_art| format!("http://images.lyricfind.com/images/{cover_art}"))
                    .as_deref()
                    .unwrap_or(""),
            )
            .set_author(&self.artist.name, None::<String>, None::<String>)
            .set_title(&self.title)
            .set_url(format!("https://lyrics.lyricfind.com/lyrics/{}", self.slug));

        if let Some(context) = self.context.as_ref() {
            embed = embed.add_field(
                "Match",
                format!("...{}...", Document::from(&context.replace("<em>", "<em>**").replace("</em>", "**</em>")).select("body").text()),
                false,
            );
        } else {
            embed = embed.add_field("Snippet", &self.snippet, false);
        }

        let mut links = vec![];

        if let Some(spotify) = self.spotify.as_ref() {
            links.push(format!("[Spotify](https://open.spotify.com/track/{spotify})"));
        }

        if let Some(apple) = self.apple.as_ref() {
            links.push(format!("[Apple Music](https://music.apple.com/en/song/{apple})"));
        }

        if let Some(deezer) = self.deezer.as_ref() {
            links.push(format!("[Deezer](https://www.deezer.com/en/track/{deezer})"));
        }

        if !links.is_empty() {
            embed = embed.add_field("Stream", links.join("\n"), false);
        }

        embed
    }
}
