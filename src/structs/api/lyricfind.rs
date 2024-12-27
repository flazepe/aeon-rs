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
#[allow(dead_code)]
pub struct LyricFindSearchResultTrack {
    pub title: String,

    #[serde(rename = "camelCase")]
    pub title_simple: Option<String>,

    #[serde(rename = "camelCase")]
    pub title_romanized: Option<String>,

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

    pub snippet: Option<String>,
    pub context: Option<String>,
    pub last_update: Option<String>,
    pub score: f64,
    pub slug: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct LyricFindSearchResultArtist {
    pub name: String,
    pub name_romanized: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct LyricFindSearchResultAlbum {
    pub title: String,
    pub release_year: Option<i64>,
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
        let thumbnail = self
            .album
            .as_ref()
            .and_then(|album| album.cover_art.as_ref())
            .map(|cover_art| format!("http://images.lyricfind.com/images/{cover_art}"))
            .unwrap_or_else(|| "".into());
        let author = &self.artist.name;
        let title = &self.title;
        let url = format!("https://lyrics.lyricfind.com/lyrics/{}", self.slug);

        let mut embed = Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(thumbnail)
            .set_author(author, None::<String>, None::<String>)
            .set_title(title)
            .set_url(url);

        if let Some(context) = &self.context {
            let mut match_text = context.replace("<em>", "<em>**").replace("</em>", "**</em>");
            match_text = format!("...{}...", Document::from(&match_text).select("body").text());
            embed = embed.add_field("Match", match_text, false);
        } else if let Some(snippet) = &self.snippet {
            embed = embed.add_field("Snippet", snippet, false);
        }

        let mut links = vec![];

        if let Some(spotify) = &self.spotify {
            links.push(format!("[Spotify](https://open.spotify.com/track/{spotify})"));
        }

        if let Some(apple) = self.apple {
            links.push(format!("[Apple Music](https://music.apple.com/en/song/{apple})"));
        }

        if let Some(deezer) = self.deezer {
            links.push(format!("[Deezer](https://www.deezer.com/en/track/{deezer})"));
        }

        if !links.is_empty() {
            embed = embed.add_field("Stream", links.join("\n"), false);
        }

        embed
    }
}
