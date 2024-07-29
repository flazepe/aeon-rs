use crate::statics::{colors::PRIMARY_COLOR, REQWEST};
use anyhow::{bail, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::{collections::HashMap, fmt::Display};

#[derive(Deserialize, Debug)]
pub struct JishoJapanese {
    pub reading: Option<String>,
    pub word: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct JishoLink {
    pub text: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct JishoSource {
    pub language: String,
    pub word: String,
}

#[derive(Deserialize, Debug)]
pub struct JishoSense {
    pub english_definitions: Vec<String>,
    pub parts_of_speech: Vec<String>,
    pub links: Vec<JishoLink>,
    pub tags: Vec<String>,
    pub see_also: Vec<String>,
    pub source: Vec<JishoSource>,
    pub info: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct JishoAttribution {
    pub jmdict: bool,
    pub jmnedict: bool,
    // pub dbpedia: String,
}

#[derive(Deserialize, Debug)]
struct JishoSearchResult {
    data: Vec<JishoSearch>,
}

#[derive(Deserialize, Debug)]
pub struct JishoSearch {
    pub slug: String,
    pub is_common: Option<bool>,
    pub tags: Vec<String>,
    pub jlpt: Vec<String>,
    pub japanese: Vec<JishoJapanese>,
    pub senses: Vec<JishoSense>,
    pub attribution: JishoAttribution,
}

impl JishoSearch {
    pub async fn get<T: Display>(slug: T) -> Result<Self> {
        let mut results = REQWEST
            .get("https://jisho.org/api/v1/search/words")
            .query(&[("slug", slug.to_string())])
            .send()
            .await?
            .json::<JishoSearchResult>()
            .await?
            .data;

        match results.is_empty() {
            true => bail!("No results found."),
            false => Ok(results.remove(0)),
        }
    }

    pub async fn search<T: Display>(query: T) -> Result<Vec<Self>> {
        let results = REQWEST
            .get("https://jisho.org/api/v1/search/words")
            .query(&[("keyword", query.to_string())])
            .send()
            .await?
            .json::<JishoSearchResult>()
            .await?
            .data;

        match results.is_empty() {
            true => bail!("No results found."),
            false => Ok(results),
        }
    }

    pub fn format_title(&self) -> String {
        let title = self.japanese[0].word.as_ref().unwrap_or_else(|| self.japanese[0].reading.as_ref().unwrap()).to_string(); // One of these gotta exist
        let reading = self.japanese[0].reading.as_deref().unwrap_or("");

        match title == reading || reading.is_empty() {
            true => title,
            false => format!("{title} （{reading}）"),
        }
    }

    pub fn format(&self) -> Embed {
        let title = self.format_title();
        let url = format!("https://jisho.org/word/{}", self.slug);
        let description = {
            let mut parts_of_speech = HashMap::new();

            for sense in &self.senses {
                let part_of_speech = match sense.parts_of_speech.is_empty() {
                    true => "others".into(),
                    false => sense.parts_of_speech.join(", ").to_lowercase(),
                };

                if !parts_of_speech.contains_key(&part_of_speech) {
                    parts_of_speech.insert(part_of_speech.clone(), vec![]);
                }

                parts_of_speech.get_mut(&part_of_speech).unwrap().push(sense.english_definitions.join(", "));
            }

            parts_of_speech
        }
        .iter()
        .map(|(k, v)| format!("{}\n{}", k, v.iter().map(|entry| format!(" - {entry}")).collect::<Vec<String>>().join("\n")))
        .collect::<Vec<String>>()
        .join("\n\n");

        Embed::new().set_color(PRIMARY_COLOR).unwrap_or_default().set_title(title).set_url(url).set_description(description)
    }
}
