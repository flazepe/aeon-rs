use crate::{
    statics::{colors::*, *},
    *,
};
use anyhow::{bail, Result};
use reqwest::get;
use serde::Deserialize;
use slashook::structs::embeds::Embed;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct SauceNAOHeader {
    pub similarity: String,
    pub thumbnail: String,
    pub index_id: u64,
    pub index_name: String,
    pub dupes: u64,
    pub hidden: u64,
}

#[derive(Deserialize)]
pub struct SauceNAOData {
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

#[derive(Deserialize)]
pub struct SauceNAOResult {
    pub header: SauceNAOHeader,
    pub data: SauceNAOData,
}

#[derive(Deserialize)]
pub struct SauceNAOSearch {
    pub results: Vec<SauceNAOResult>,
}

impl SauceNAOSearch {
    pub async fn query<T: Display>(url: T) -> Result<Self> {
        let search = get(format!(
            "https://saucenao.com/search.php?api_key={}&output_type=2&url={url}",
            CONFIG.api.saucenao_key
        ))
        .await?
        .json::<Self>()
        .await?;

        if search.results.is_empty() {
            bail!("Sauce not found.");
        }

        Ok(search)
    }

    pub fn format(self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_description(
                self.results
                    .iter()
                    .map(|result| {
                        format!(
                            "`[{}%]` [{}]({}){}",
                            result.header.similarity,
                            {
                                let title: String;

                                if result.data.pixiv_id.is_some() {
                                    title = "Pixiv Source".into();
                                } else if result.data.gelbooru_id.is_some() {
                                    title = "Gelbooru Source".into();
                                } else {
                                    title = and_then_or!(
                                        result.data.source.as_ref(),
                                        |source| Some(source.into()),
                                        "".into()
                                    );
                                }

                                if_else!(title.is_empty(), "Source".into(), title)
                            },
                            result
                                .data
                                .ext_urls
                                .as_ref()
                                .unwrap_or(&vec!["https://google.com".into()])[0],
                            {
                                let joined = [
                                    result.data.year.as_ref().unwrap_or(&"".into()).into(),
                                    match &result.data.part {
                                        Some(part) => format!(
                                            "{} {}",
                                            if_else!(
                                                part.chars().into_iter().all(|char| char.is_numeric()),
                                                "Episode",
                                                ""
                                            ),
                                            part.replace('-', "").trim()
                                        ),
                                        None => "".into(),
                                    },
                                    result.data.est_time.as_ref().unwrap_or(&"".into()).into(),
                                ]
                                .into_iter()
                                .filter(|entry| !entry.is_empty())
                                .collect::<Vec<String>>()
                                .join("\n");

                                if_else!(joined.is_empty(), "".into(), format!("\n{joined}"))
                            }
                        )
                    })
                    .take(5)
                    .collect::<Vec<String>>()
                    .join("\n\n"),
            )
    }
}
