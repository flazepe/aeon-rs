pub mod statics;

use crate::structs::unicode::statics::CONTROL_CHARACTERS;
use anyhow::{bail, Context, Result};
use nipper::Document;
use reqwest::Client;
use std::fmt::Display;
use unicode_names2::name as get_unicode_name;

pub struct Unicode(Vec<UnicodeCharacter>);

impl Unicode {
    pub async fn search<T: Display>(name: T) -> Result<Self> {
        let document =
            Document::from(&Client::new().get("https://symbl.cc/en/search/").query(&[("q", name.to_string())]).send().await?.text().await?);

        let name = document.select("h2").first().text().trim().to_string();

        if name.is_empty() {
            bail!("Unicode character not found.");
        }

        let character = document.select(".search-page__char").first().text().to_string().trim().to_string();

        Ok(Self(vec![UnicodeCharacter {
            codepoint: format!("U+{:04X}", character.chars().next().context("Empty string provided.")? as u32),
            name,
            character,
        }]))
    }

    pub fn list<T: Display>(string: T) -> Self {
        let mut unicode_characters: Vec<UnicodeCharacter> = vec![];

        for character in string.to_string().chars() {
            let codepoint = format!("U+{:04X}", character as u32);

            if unicode_characters.iter().any(|unicode_character| unicode_character.codepoint == codepoint) {
                continue;
            }

            let mut name = "UNKNOWN".into();

            if let Some(character_name) = CONTROL_CHARACTERS.get(format!("{:X}", character as u32).as_str()) {
                name = character_name.to_string();
            }

            if let Some(character_name) = get_unicode_name(character) {
                name = character_name.to_string();
            }

            unicode_characters.push(UnicodeCharacter { codepoint, name, character: character.to_string() });
        }

        Self(unicode_characters)
    }

    pub fn format(&self) -> String {
        self.0
            .iter()
            .map(|character| {
                format!(
                    "[`{codepoint}`](<https://www.compart.com/en/unicode/{codepoint}>) {}",
                    character.name,
                    codepoint = character.codepoint,
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub struct UnicodeCharacter {
    pub codepoint: String,
    pub name: String,
    pub character: String,
}
