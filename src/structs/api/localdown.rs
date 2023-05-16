use crate::{
    statics::{colors::PRIMARY_COLOR, CACHE, REQWEST},
    traits::LimitedVec,
};
use anyhow::{bail, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;

#[derive(Clone, Deserialize)]
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

#[derive(Deserialize)]
pub struct LocalDownSearchResult {
    pub cover_url: String,
    pub id: u64,
    pub title: String,
}

impl LocalDownNovel {
    pub async fn search<T: ToString>(query: T) -> Result<Vec<LocalDownSearchResult>> {
        let results = REQWEST
            .get("https://api.ahnafzamil.com/localdown/novels/search")
            .query(&[("q", query.to_string().as_str())])
            .header("user-agent", "yes")
            .send()
            .await?
            .json::<Vec<LocalDownSearchResult>>()
            .await?;

        if results.is_empty() {
            bail!("Novel not found.");
        }

        Ok(results)
    }

    pub async fn get(id: u64) -> Result<Self> {
        if let Some(novel) = CACHE.localdown_novels.read().unwrap().iter().find(|novel| novel.id == id) {
            return Ok(novel.clone());
        }

        match REQWEST
            .get(format!("https://api.ahnafzamil.com/localdown/novels/get/{id}"))
            .header("user-agent", "yes")
            .send()
            .await?
            .json::<Self>()
            .await
        {
            Ok(novel) => {
                CACHE.localdown_novels.write().unwrap().push_limited(novel.clone(), 100);
                Ok(novel)
            },
            Err(_) => bail!("Novel not found."),
        }
    }

    pub fn format(&self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_thumbnail(&self.cover_url)
            .set_title(format!("{} ({})", self.title, self.start_year))
            .set_url(format!(
                "https://www.novelupdates.com/series/{}/",
                self.title
                    .to_lowercase()
                    .chars()
                    .map(|char| match [' ', '-'].contains(&char) {
                        true => '-',
                        false => match char.is_ascii_alphabetic() {
                            true => char,
                            false => '_',
                        },
                    })
                    .filter(|char| char != &'_')
                    .collect::<String>()
            ))
            .set_description(
                self.other_names
                    .chars()
                    .skip(1)
                    .take(self.other_names.len() - 2)
                    .collect::<String>()
                    .split("', '")
                    .map(|entry| format!("_{entry}_"))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
            .add_field("Genre", &self.genres, false)
            .add_field("Publisher", &self.publisher, false)
    }
}
