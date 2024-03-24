use crate::{functions::limit_strings, statics::REQWEST};
use anyhow::{bail, Result};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct UrbanDictionary {
    pub list: Vec<UrbanDictionaryEntry>,
}

#[derive(Clone, Deserialize)]
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
    pub async fn search<T: ToString>(word: T) -> Result<Self> {
        let result = REQWEST
            .get("http://api.urbandictionary.com/v0/define")
            .query(&[("term", word.to_string())])
            .send()
            .await?
            .json::<Self>()
            .await?;

        match result.list.is_empty() {
            true => bail!("Word not found."),
            false => Ok(result),
        }
    }

    pub fn format(&self) -> String {
        let word = self.list[0].word.to_lowercase();

        let mut cloned_list = self.list.clone();
        cloned_list.sort_by(|a, b| b.thumbs_up.cmp(&a.thumbs_up));

        format!(
            "# [{word}](<https://www.urbandictionary.com/define.php?term={}>)\n[urban]\n{}",
            word.replace(' ', "+"),
            limit_strings(
                cloned_list.iter().map(|meaning| format!("- {}", meaning.definition.split('\n').next().unwrap().replace(['[', ']'], ""))),
                "\n",
                1900,
            ),
        )
    }
}
