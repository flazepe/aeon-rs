mod statics;

use crate::{
    statics::{colors::PRIMARY_COLOR, CACHE, REQWEST},
    traits::LimitedVec,
};
use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::from_str;
use slashook::structs::embeds::Embed;
use statics::GENRES;
use std::fmt::Display;

#[derive(Deserialize, Clone, Debug)]
pub struct LocalDownNovel {
    pub authors: String,
    pub cover_url: String,
    pub genres: String,
    pub id: u64,
    pub other_names: String,
    pub publisher: String,
    pub start_year: u64,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct LocalDownNovelSearchResult {
    pub cover_url: String,
    pub id: u64,
    pub title: String,
}

impl LocalDownNovel {
    pub async fn search<T: Display>(query: T) -> Result<Vec<LocalDownNovelSearchResult>> {
        let results = REQWEST
            .get("https://api.ahnafzamil.com/localdown/novels/search")
            .query(&[("q", query.to_string())])
            .header("user-agent", "yes")
            .send()
            .await?
            .json::<Vec<LocalDownNovelSearchResult>>()
            .await?;

        match results.is_empty() {
            true => bail!("Novel not found."),
            false => Ok(results),
        }
    }

    pub async fn get(id: u64) -> Result<Self> {
        if let Some(novel) = CACHE.localdown_novels.read().unwrap().iter().find(|novel| novel.id == id) {
            return Ok(novel.clone());
        }

        let Ok(novel) = REQWEST
            .get(format!("https://api.ahnafzamil.com/localdown/novels/get/{id}"))
            .header("user-agent", "yes")
            .send()
            .await?
            .json::<Self>()
            .await
        else {
            bail!("Novel not found.")
        };

        CACHE.localdown_novels.write().unwrap().push_limited(novel.clone(), 100);
        Ok(novel)
    }

    pub fn format(&self) -> Embed {
        let thumbnail = &self.cover_url;
        let title = format!(
            "{} ({})",
            match self.title.len() > 249 {
                true => format!("{}…", self.title.chars().take(248).collect::<String>().trim()),
                false => self.title.clone(),
            },
            self.start_year,
        );
        let url = format!(
            "https://www.novelupdates.com/series/{}/",
            self.title
                .to_lowercase()
                .chars()
                .map(|char| match [' ', '-'].contains(&char) {
                    true => '-',
                    false => match char.is_ascii_alphanumeric() {
                        true => char,
                        false => '_',
                    },
                })
                .filter(|char| char != &'_')
                .collect::<String>(),
        );
        let other_names = from_str::<Vec<String>>(
            format!(
                r#"["{}"]"#,
                self.other_names
                    .chars()
                    .skip(1)
                    .take(2.max(self.other_names.chars().count()) - 2)
                    .collect::<String>()
                    .replace("', ", r#"", "#)
                    .replace(", '", r#", ""#),
            )
            .as_str(),
        )
        .unwrap_or_default()
        .iter()
        .map(|name| format!("_{name}_"))
        .collect::<Vec<String>>()
        .join("\n");
        let genres = self.genres
            .split(", ")
            .map(|genre| GENRES.get(&genre).unwrap_or(&"").to_string()) // unwrap_or()'d just in case
            .filter(|genre| !genre.is_empty()) // Edge case
            .collect::<Vec<String>>()
            .join(", ");
        let publisher = &self.publisher;

        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(thumbnail)
            .set_title(title)
            .set_url(url)
            .set_description(other_names)
            .add_field("Genre", genres, false)
            .add_field("Publisher", publisher, false)
            .set_footer("Powered by Project LocalDown API", None::<String>)
    }
}
