use crate::statics::{colors::PRIMARY_COLOR, CONFIG, REQWEST};
use anyhow::{bail, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
pub struct SauceNaoHeader {
    pub similarity: String,
    pub thumbnail: String,
    pub index_id: u64,
    pub index_name: String,
    pub dupes: u64,
    pub hidden: u64,
}

#[derive(Deserialize, Debug)]
pub struct SauceNaoData {
    pub est_time: Option<String>,
    pub ext_urls: Option<Vec<String>>,
    pub gelbooru_id: Option<u64>,
    pub mal_id: Option<u64>,
    pub md_id: Option<String>,
    pub part: Option<String>,
    pub pixiv_id: Option<u64>,
    pub source: Option<String>,
    pub year: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SauceNaoResult {
    pub header: SauceNaoHeader,
    pub data: SauceNaoData,
}

#[derive(Deserialize, Debug)]
pub struct SauceNaoSearch {
    pub results: Vec<SauceNaoResult>,
}

impl SauceNaoSearch {
    pub async fn query<T: Display>(url: T) -> Result<Self> {
        let search = REQWEST
            .get("https://saucenao.com/search.php")
            .query(&[("api_key", CONFIG.api.saucenao_key.as_str()), ("output_type", "2"), ("url", url.to_string().as_str())])
            .send()
            .await?
            .json::<Self>()
            .await?;

        if search.results.is_empty() {
            bail!("Sauce not found.");
        }

        Ok(search)
    }

    pub fn format(&self) -> Embed {
        Embed::new().set_color(PRIMARY_COLOR).unwrap_or_default().set_description(
            self.results
                .iter()
                .map(|result| {
                    format!(
                        "`[{}%]` [{}]({}){}",
                        result.header.similarity,
                        {
                            let title;

                            if result.data.pixiv_id.is_some() {
                                title = "Pixiv Source";
                            } else if result.data.gelbooru_id.is_some() {
                                title = "Gelbooru Source";
                            } else {
                                title = result.data.source.as_deref().unwrap_or("");
                            }

                            match title.is_empty() {
                                true => "Source",
                                false => title,
                            }
                        },
                        result.data.ext_urls.as_ref().unwrap_or(&vec![]).first().map_or("https://google.com", |url| url.as_str()),
                        {
                            let joined = [
                                result.data.year.as_deref().unwrap_or("").into(),
                                match &result.data.part {
                                    Some(part) => format!(
                                        "{} {}",
                                        match part.chars().all(|char| char.is_numeric()) {
                                            true => "Episode",
                                            false => "",
                                        },
                                        part.replace('-', "").trim(),
                                    ),
                                    None => "".into(),
                                },
                                result.data.est_time.as_deref().unwrap_or("").into(),
                            ]
                            .into_iter()
                            .filter(|entry| !entry.is_empty())
                            .collect::<Vec<String>>()
                            .join("\n");

                            match joined.is_empty() {
                                true => "".into(),
                                false => format!("\n{joined}"),
                            }
                        },
                    )
                })
                .take(5)
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }
}
