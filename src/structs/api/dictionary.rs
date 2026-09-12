use crate::{functions::limit_strings, statics::REQWEST};
use anyhow::{Error, Result};
use serde::Deserialize;
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
    pub senses: Vec<DictionaryDefinition>,
}

#[derive(Deserialize, Debug)]
pub struct DictionaryDefinition {
    pub definition: String,
}

impl Dictionary {
    pub async fn search<T: Display>(word: T) -> Result<Self> {
        REQWEST
            .get(format!("https://freedictionaryapi.com/api/v1/entries/en/{word}"))
            .send()
            .await?
            .json::<Self>()
            .await
            .map_err(|_| Error::msg("Word not found."))
    }

    pub fn format(&self) -> String {
        let entry = &self.entries[0];

        format!(
            "# {}\n[{}]\n{}",
            self.word,
            entry.part_of_speech,
            limit_strings(entry.senses.iter().map(|meaning| format!("- {}", meaning.definition)), "\n", 1900),
        )
    }
}
