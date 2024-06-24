use crate::statics::{colors::PRIMARY_COLOR, REQWEST};
use anyhow::{bail, Result};
use nipper::Document;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

pub struct AnimeSongLyrics {
    pub title: String,
    pub url: String,
    pub anime: String,
    pub cover: Option<String>,
}

impl AnimeSongLyrics {
    pub async fn query<T: Display>(query: T) -> Result<Self> {
        let document = Document::from(
            &REQWEST.get("https://animesonglyrics.com/results").query(&[("q", query.to_string())]).send().await?.text().await?,
        );

        let selection = document.select("#songlist a");

        let node = match selection.get(0) {
            Some(node) => node,
            None => bail!("Anime not found."),
        };

        let data = node.text();
        let mut data = data.split('\n').filter(|str| !str.is_empty());

        let Some(anime) = data.next() else {
            bail!("Could not get song anime.");
        };

        let Some(title) = data.next() else {
            bail!("Could not get song title.");
        };

        Ok(Self {
            title: title.trim_start_matches(['-', ':']).trim().to_string(),
            url: node.attr("href").map_or_else(|| "https://animesonglyrics.com".into(), |href| href.to_string()),
            anime: anime.to_string(),
            cover: document.select("#songlist img").attr("data-src").map(|src| src.to_string()),
        })
    }

    pub fn format(&self) -> Embed {
        let mut embed = Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_title(&self.title)
            .set_url(&self.url)
            .set_description(&self.anime);

        if let Some(cover) = self.cover.as_ref() {
            embed = embed.set_thumbnail(cover);
        }

        embed
    }
}
