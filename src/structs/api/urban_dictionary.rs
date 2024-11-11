use crate::{functions::limit_strings, statics::REQWEST};
use anyhow::{bail, Result};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Clone, Debug)]
pub struct UrbanDictionary {
    pub list: Vec<UrbanDictionaryEntry>,
}

#[derive(Deserialize, Clone, Debug)]
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

    pub fn format(&self) -> String {
        let word = self.list[0].word.to_lowercase();

        let mut list = self.list.clone();
        list.sort_by(|a, b| b.thumbs_up.cmp(&a.thumbs_up));

        format!(
            "# [{word}](<https://www.urbandictionary.com/define.php?term={}>)\n[urban]\n{}",
            word.replace(' ', "+"),
            limit_strings(
                list.iter().map(|meaning| format!("- {}", meaning.definition.split('\n').next().unwrap().replace(['[', ']'], ""))),
                "\n",
                1900,
            ),
        )
    }
}
