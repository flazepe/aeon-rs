use crate::statics::{colors::PRIMARY_COLOR, CONFIG, REQWEST};
use anyhow::{bail, Result};
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SauceNaoHeader {
    pub similarity: String,
    pub thumbnail: String,
    pub index_id: u64,
    pub index_name: String,
    pub dupes: u64,
    pub hidden: u64,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
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
    pub results: Option<Vec<SauceNaoResult>>,
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

        if search.results.as_ref().map_or(true, |results| results.is_empty()) {
            bail!("Sauce not found.");
        }

        Ok(search)
    }

    pub fn format(&self) -> Embed {
        let description = if let Some(results) = &self.results {
            results
                .iter()
                .take(5)
                .map(|result| {
                    let additional_info = [
                        result.data.year.as_deref().unwrap_or("").into(),
                        result.data.part.as_ref().map_or_else(
                            || "".into(),
                            |part| {
                                format!(
                                    "{} {}",
                                    if part.chars().all(|char| char.is_numeric()) { "Episode" } else { "" },
                                    part.replace('-', "").trim(),
                                )
                            },
                        ),
                        result.data.est_time.as_deref().unwrap_or("").into(),
                    ]
                    .into_iter()
                    .filter(|entry| !entry.is_empty())
                    .collect::<Vec<String>>()
                    .join("\n");

                    format!(
                        "`[{}%]` [{}]({}){}",
                        result.header.similarity,
                        result.header.index_name,
                        result
                            .data
                            .ext_urls
                            .as_ref()
                            .and_then(|ext_urls| ext_urls.first())
                            .map_or("https://google.com", |url| url.as_str()),
                        if additional_info.is_empty() { "".into() } else { format!("\n{additional_info}\n") },
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            "-".into()
        };

        Embed::new().set_color(PRIMARY_COLOR).unwrap_or_default().set_description(description)
    }
}
