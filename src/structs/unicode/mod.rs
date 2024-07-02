pub mod statics;

use crate::structs::unicode::statics::CONTROL_CHARACTERS;
use anyhow::{bail, Result};
use nipper::Document;
use reqwest::Client;
use std::fmt::Display;
use unicode_names2::name as get_unicode_name;

pub struct Unicode(Vec<UnicodeCharacter>);

impl Unicode {
    pub fn list<T: Display>(string: T) -> Self {
        let mut characters: Vec<UnicodeCharacter> = vec![];

        for char in string.to_string().chars() {
            let codepoint = format!("U+{:04X}", char as u32);

            if characters.iter().any(|character| character.codepoint == codepoint) {
                continue;
            }

            characters.push(UnicodeCharacter { codepoint, name: Self::get_name(char), character: char.to_string() });
        }

        Self(characters)
    }

    pub async fn search<T: Display>(name: T) -> Result<Self> {
        let document = Document::from(
            &Client::new()
                .get("https://www.fileformat.info/info/unicode/char/search.htm")
                .query(&[("q", name.to_string())])
                .send()
                .await?
                .text()
                .await?,
        );

        let text = document.select("td:nth-child(4)").text();

        if text.is_empty() {
            bail!("Unicode character not found.");
        }

        Ok(Self::list(text))
    }

    fn get_name(char: char) -> String {
        if let Some(name) = CONTROL_CHARACTERS.get(format!("{:X}", char as u32).as_str()) {
            return name.to_string();
        }

        if let Some(name) = get_unicode_name(char) {
            return name.to_string();
        }

        "UNKNOWN".to_string()
    }

    pub fn format(&self) -> String {
        self.0
            .iter()
            .map(|character| {
                format!(
                    "[`{}`](<https://www.fileformat.info/info/unicode/char/{}/index.htm>) {}",
                    character.codepoint,
                    character.codepoint.chars().skip(2).collect::<String>().to_lowercase(),
                    character.name,
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
