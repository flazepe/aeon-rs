use crate::statics::{REQWEST, colors::PRIMARY_COLOR};
use anyhow::{Result, bail};
use nipper::Document;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Debug)]
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
        let Some(node) = selection.get(0) else { bail!("Anime not found.") };

        let data = node.text();
        let mut data = data.split('\n').map(|str| str.trim()).filter(|str| !str.is_empty());

        let Some(anime) = data.next() else { bail!("Could not get song anime.") };
        let Some(title) = data.next() else { bail!("Could not get song title.") };

        Ok(Self {
            title: title.trim_start_matches(['-', ':']).trim().to_string(),
            url: node.attr("href").map_or_else(|| "https://animesonglyrics.com".into(), |href| href.to_string()),
            anime: anime.to_string(),
            cover: document.select("#songlist img").attr("data-src").map(|src| src.to_string()),
        })
    }

    pub fn format(&self) -> Embed {
        let image = self.cover.as_deref().unwrap_or("");
        let title = &self.title;
        let url = &self.url;
        let anime = &self.anime;

        Embed::new().set_color(PRIMARY_COLOR).unwrap_or_default().set_image(image).set_title(anime).set_url(url).set_description(title)
    }
}
