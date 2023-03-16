use crate::constants::CONTROL_CHARACTERS;
use anyhow::{bail, Context, Result};
use nipper::Document;
use reqwest::get;
use unicode_names2::name as get_unicode_name;

pub struct UnicodeCharacter {
    codepoint: String,
    name: String,
    character: String,
}

pub struct UnicodeCharacters {
    pub unicode_characters: Vec<UnicodeCharacter>,
}

impl UnicodeCharacter {
    pub async fn get(name: &str) -> Result<Self> {
        let document = Document::from(
            &get(format!("https://symbl.cc/en/search/?q={name}"))
                .await?
                .text()
                .await?,
        );

        let name = document.select("h2").first().text().trim().to_string();

        if name.is_empty() {
            bail!("Unicode character not found.");
        }

        let character = document
            .select(".search-page__char")
            .first()
            .text()
            .to_string()
            .trim()
            .to_string();

        Ok(Self {
            codepoint: format!(
                "`U+{:04X}`",
                character.chars().next().context("Empty string")? as u32,
            ),
            name,
            character,
        })
    }

    pub fn format(self) -> String {
        format!(
            "`{}` - {}{}",
            self.codepoint,
            self.name,
            if CONTROL_CHARACTERS
                .iter()
                .any(|[_, control_character]| control_character == &self.name)
            {
                String::from("")
            } else {
                format!(" - `{}`", self.character.replace('`', "｀"))
            }
        )
    }
}

impl UnicodeCharacters {
    pub fn get(string: &str) -> Self {
        let mut unicode_characters: Vec<UnicodeCharacter> = vec![];

        for character in string.chars() {
            let codepoint = format!("U+{:04X}", character as u32);
            let mut name = String::from("UNKNOWN");

            if let Some(character_name) = CONTROL_CHARACTERS.iter().find(|[control_character, _]| {
                control_character == &format!("{:X}", character as u32)
            }) {
                name = character_name[1].to_string();
            }

            if let Some(character_name) = get_unicode_name(character) {
                name = character_name.to_string();
            }

            unicode_characters.push(UnicodeCharacter {
                codepoint,
                name,
                character: character.to_string(),
            });
        }

        Self { unicode_characters }
    }

    pub fn format(self) -> String {
        let unicode_characters = self
            .unicode_characters
            .into_iter()
            .take(20)
            .collect::<Vec<UnicodeCharacter>>();

        format!(
            "Showing first {} character{}:\n\n{}",
            unicode_characters.len(),
            if unicode_characters.len() == 1 {
                ""
            } else {
                "s"
            },
            unicode_characters
                .into_iter()
                .map(|unicode_character| { unicode_character.format() })
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
