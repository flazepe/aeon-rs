use crate::{macros::if_else, statics::colors::PRIMARY_COLOR};
use anyhow::{bail, Result};
use reqwest::Client;
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::{collections::HashMap, fmt::Display};

#[derive(Deserialize)]
pub struct JishoJapanese {
    pub reading: Option<String>,
    pub word: Option<String>,
}

#[derive(Deserialize)]
pub struct JishoLink {
    pub text: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct JishoSource {
    pub language: String,
    pub word: String,
}

#[derive(Deserialize)]
pub struct JishoSense {
    pub english_definitions: Vec<String>,
    pub parts_of_speech: Vec<String>,
    pub links: Vec<JishoLink>,
    pub tags: Vec<String>,
    pub see_also: Vec<String>,
    pub source: Vec<JishoSource>,
    pub info: Vec<String>,
}

#[derive(Deserialize)]
pub struct JishoAttribution {
    pub jmdict: bool,
    pub jmnedict: bool,
    // pub dbpedia: String,
}

#[derive(Deserialize)]
pub struct JishoSearch {
    pub slug: String,
    pub is_common: Option<bool>,
    pub tags: Vec<String>,
    pub jlpt: Vec<String>,
    pub japanese: Vec<JishoJapanese>,
    pub senses: Vec<JishoSense>,
    pub attribution: JishoAttribution,
}

#[derive(Deserialize)]
pub struct JishoSearchResult {
    pub data: Vec<JishoSearch>,
}

impl JishoSearch {
    pub async fn get<T: Display>(slug: T) -> Result<Self> {
        let mut results = Client::new()
            .get("https://jisho.org/api/v1/search/words")
            .query(&[("slug", slug.to_string().as_str())])
            .send()
            .await?
            .json::<JishoSearchResult>()
            .await?
            .data;

        if_else!(results.is_empty(), bail!("No results found."), Ok(results.remove(0)))
    }

    pub async fn search<T: ToString>(query: T) -> Result<Vec<Self>> {
        let results = Client::new()
            .get("https://jisho.org/api/v1/search/words")
            .query(&[("keyword", query.to_string().as_str())])
            .send()
            .await?
            .json::<JishoSearchResult>()
            .await?
            .data;

        if_else!(results.is_empty(), bail!("No results found."), Ok(results))
    }

    pub fn format_title(&self) -> String {
        let title = self.japanese[0].word.as_ref().unwrap_or_else(|| self.japanese[0].reading.as_ref().unwrap()).to_string(); // One of these gotta exist
        let reading = self.japanese[0].reading.as_ref().unwrap_or(&"".into()).to_string();
        if_else!(title == reading || reading.is_empty(), title, format!("{title} （{reading}）"))
    }

    pub fn format(self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_title(self.format_title())
            .set_url(format!("https://jisho.org/word/{}", self.slug))
            .set_description(
                {
                    let mut parts_of_speech = HashMap::new();

                    for sense in self.senses {
                        let part_of_speech =
                            if_else!(sense.parts_of_speech.is_empty(), "Others".into(), sense.parts_of_speech.join(", ")).to_lowercase();

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
                .join("\n\n"),
            )
    }
}
