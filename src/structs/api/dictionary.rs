use crate::{
    statics::{REQWEST, colors::PRIMARY_EMBED_COLOR},
    structs::pagination::Pagination,
};
use anyhow::{Result, ensure};
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

        ensure!(!result.entries.is_empty(), "Word not found.");
        Ok(result)
    }

    pub fn format_pagination<T: Display>(&self, command: T) -> Pagination {
        let mut pagination = Pagination::new(command);

        let part_of_speeches = self.entries.iter().map(|entry| &entry.part_of_speech).collect::<Vec<&String>>();

        for (index, entry) in self.entries.iter().enumerate() {
            let counter =
                part_of_speeches.iter().take(index).filter(|part_of_speech| ***part_of_speech == entry.part_of_speech).count() + 1;

            let should_display_counter =
                part_of_speeches.iter().filter(|part_of_speech| ***part_of_speech == entry.part_of_speech).count() > 1;

            let label = format!("{}{}", entry.part_of_speech, if should_display_counter { format!(" ({counter})") } else { "".into() });

            let description = format!(
                "-# {}\n{}",
                label,
                entry.senses.iter().map(|sense| format!("- {}", sense.definition)).collect::<Vec<String>>().join("\n"),
            );

            let embed = Embed::new().set_color(PRIMARY_EMBED_COLOR).unwrap_or_default().set_title(&self.word).set_description(description);
            pagination = pagination.add_section(label, embed);
        }

        pagination
    }
}
