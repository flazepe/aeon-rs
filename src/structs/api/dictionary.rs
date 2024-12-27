use crate::{functions::limit_strings, statics::REQWEST};
use anyhow::{Error, Result};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
pub struct Dictionary {
    list: Vec<DictionaryEntry>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DictionaryEntry {
    pub word: String,
    pub phonetic: Option<String>,
    pub phonetics: Vec<DictionaryPhonetic>,
    pub meanings: Vec<DictionaryMeaning>,
    pub license: DictionaryLicense,
    pub source_urls: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DictionaryPhonetic {
    pub text: Option<String>,
    pub audio: String,
    pub source_url: Option<String>,
    pub license: Option<DictionaryLicense>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DictionaryMeaning {
    pub part_of_speech: String,
    pub definitions: Vec<DictionaryDefinition>,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DictionaryDefinition {
    pub definition: String,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
    pub example: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DictionaryLicense {
    pub name: String,
    pub url: String,
}

impl Dictionary {
    pub async fn search<T: Display>(word: T) -> Result<Self> {
        Ok(Self {
            list: REQWEST
                .get(format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}"))
                .send()
                .await?
                .json::<Vec<DictionaryEntry>>()
                .await
                .map_err(|_| Error::msg("Word not found."))?,
        })
    }

    pub fn format(&self) -> String {
        format!(
            "# {}\n{}",
            self.list[0].word,
            limit_strings(
                self.list[0].meanings.iter().map(|meaning| format!(
                    "[{}]\n{}",
                    meaning.part_of_speech,
                    meaning.definitions.iter().map(|definition| format!("- {}", definition.definition)).collect::<Vec<String>>().join("\n"),
                )),
                "\n\n",
                1900,
            ),
        )
    }
}
