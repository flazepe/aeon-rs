use crate::constants::CONTROL_CHARACTERS;
use anyhow::{bail, Context, Result};
use nipper::Document;
use reqwest::get;
use unicode_names2::name as get_unicode_name;

pub struct UnicodeCharacter {
    pub codepoint: String,
    pub name: String,
    pub character: String,
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

    pub fn list(string: &str) -> Vec<Self> {
        let mut unicodes: Vec<Self> = vec![];

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

            unicodes.push(Self {
                codepoint,
                name,
                character: character.to_string(),
            });
        }

        unicodes
    }

    pub fn formatted_list(string: &str) -> String {
        let unicode_characters = UnicodeCharacter::list(string)
            .into_iter()
            .take(20)
            .collect::<Vec<UnicodeCharacter>>();

        format!(
            "Showing first {} character(s):\n\n{}",
            unicode_characters.len(),
            unicode_characters
                .into_iter()
                .map(|unicode_character| { unicode_character.format() })
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
