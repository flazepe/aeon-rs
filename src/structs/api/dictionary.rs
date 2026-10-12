use crate::{
    statics::{REQWEST, colors::PRIMARY_EMBED_COLOR},
    structs::pagination::{Pagination, PaginationPages, PaginationSection},
};
use anyhow::{Result, bail};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize, Default, Debug)]
pub struct Dictionary {
    word: String,
    entries: Vec<DictionaryEntry>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DictionaryEntry {
    pub part_of_speech: String,
    pub senses: Vec<DictionarySense>,
}

#[derive(Deserialize, Debug)]
pub struct DictionarySense {
    pub definition: String,
}

impl Dictionary {
    pub async fn search<T: Display>(word: T) -> Result<Self> {
        let result = REQWEST
            .get(format!("https://freedictionaryapi.com/api/v1/entries/en/{word}"))
            .send()
            .await?
            .json::<Self>()
            .await
            .unwrap_or_default();

        if result.entries.is_empty() {
            bail!("Word not found.");
        }

        Ok(result)
    }

    pub fn format_pagination<T: Display>(&self, command: T) -> Pagination {
        let pages = self
            .entries
            .iter()
            .map(|entry| {
                vec![Embed::new().set_color(PRIMARY_EMBED_COLOR).unwrap_or_default().set_title(&self.word).set_description(format!(
                    "-# {}\n{}",
                    entry.part_of_speech,
                    entry.senses.iter().map(|sense| format!("- {}", sense.definition)).collect::<Vec<String>>().join("\n"),
                ))]
            })
            .collect::<Vec<Vec<Embed>>>();

        Pagination::new(command.to_string(), vec![PaginationSection::new("Main", PaginationPages::Embeds(pages))])
    }
}
