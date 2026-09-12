use crate::{
    functions::limit_strings,
    statics::{REQWEST, colors::PRIMARY_EMBED_COLOR},
};
use anyhow::{Error, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
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
        REQWEST
            .get(format!("https://freedictionaryapi.com/api/v1/entries/en/{word}"))
            .send()
            .await?
            .json()
            .await
            .map_err(|_| Error::msg("Word not found."))
    }

    pub fn format(&self) -> Embed {
        Embed::new().set_color(PRIMARY_EMBED_COLOR).unwrap_or_default().set_title(&self.word).set_description(limit_strings(
            self.entries.iter().map(|entry| {
                format!(
                    "-# {}\n{}",
                    entry.part_of_speech,
                    entry.senses.iter().map(|sense| format!("- {}", sense.definition)).collect::<Vec<String>>().join("\n"),
                )
            }),
            "\n",
            4096,
        ))
    }
}
