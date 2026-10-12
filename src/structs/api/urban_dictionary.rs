use crate::{
    statics::{REQWEST, colors::PRIMARY_EMBED_COLOR},
    structs::pagination::{Pagination, PaginationPages, PaginationSection},
};
use anyhow::{Result, bail};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize, Clone, Debug)]
pub struct UrbanDictionary {
    pub list: Vec<UrbanDictionaryEntry>,
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct UrbanDictionaryEntry {
    pub author: String,
    pub current_vote: String,
    pub defid: u64,
    pub definition: String,
    pub example: String,
    pub permalink: String,
    pub thumbs_down: u64,
    pub thumbs_up: u64,
    pub word: String,
    pub written_on: String,
}

impl UrbanDictionary {
    pub async fn search<T: Display>(word: T) -> Result<Self> {
        let result = REQWEST
            .get("http://api.urbandictionary.com/v0/define")
            .query(&[("term", word.to_string())])
            .send()
            .await?
            .json::<Self>()
            .await?;

        if result.list.is_empty() {
            bail!("Word not found.");
        }

        Ok(result)
    }

    pub fn format_pagination<T: Display>(&self, command: T) -> Pagination {
        let word = self.list[0].word.to_lowercase();

        let mut list = self.list.clone();
        list.sort_by_key(|entry| entry.thumbs_up);

        let pages = list
            .iter()
            .map(|meaning| {
                vec![
                    Embed::new()
                        .set_color(PRIMARY_EMBED_COLOR)
                        .unwrap_or_default()
                        .set_title(&word)
                        .set_url(format!("https://www.urbandictionary.com/define.php?term={}", word.replace(' ', "+")))
                        .set_description(meaning.definition.split('\n').next().unwrap().replace(['[', ']'], "")),
                ]
            })
            .collect::<Vec<Vec<Embed>>>();

        Pagination::new(command, vec![PaginationSection::new("Main", PaginationPages::Embeds(pages))])
    }
}
