pub mod statics;

use crate::{
    functions::paginate_strings,
    statics::colors::PRIMARY_EMBED_COLOR,
    structs::{
        pagination::{Pagination, PaginationPages, PaginationSection},
        unicode::statics::CONTROL_CHARACTERS,
    },
};
use anyhow::{Result, bail};
use nipper::Document;
use reqwest::Client;
use slashook::structs::embeds::Embed;
use std::fmt::Display;
use unicode_names2::name as get_unicode_name;

pub struct Unicode(Vec<UnicodeCharacter>);

impl Unicode {
    pub fn list<T: Display>(string: T) -> Self {
        let mut characters: Vec<UnicodeCharacter> = vec![];

        for char in string.to_string().chars() {
            if characters.iter().any(|character| character.0 == char) {
                continue;
            }

            characters.push(UnicodeCharacter(char));
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

    pub fn format_pagination<T: Display>(&self, command: T) -> Pagination {
        let paginated = paginate_strings(self.0.iter().map(|character| character.format()), "\n", 4096);

        let pages = paginated
            .iter()
            .map(|entry| vec![Embed::new().set_color(PRIMARY_EMBED_COLOR).unwrap_or_default().set_description(entry)])
            .collect::<Vec<Vec<Embed>>>();

        Pagination::new(command, vec![PaginationSection::new("Main", PaginationPages::Embeds(pages))])
    }
}

#[derive(Debug)]
pub struct UnicodeCharacter(char);

impl UnicodeCharacter {
    fn get_name(&self) -> String {
        if let Some(name) = CONTROL_CHARACTERS.get(format!("{:X}", self.0 as u32).as_str()) {
            return name.to_string();
        }

        if let Some(name) = get_unicode_name(self.0) {
            return name.to_string();
        }

        "UNKNOWN".to_string()
    }

    fn format(&self) -> String {
        let codepoint = format!("U+{:04X}", self.0 as u32);

        format!(
            "`{codepoint}` - [{}](<https://www.fileformat.info/info/unicode/char/{}/index.htm>)",
            self.get_name(),
            codepoint.trim_start_matches("U+"),
        )
    }
}
