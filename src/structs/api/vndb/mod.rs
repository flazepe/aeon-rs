pub mod character;
pub mod character_trait;
pub mod tag;
pub mod visual_novel;

use anyhow::Result;
use regex::{Regex, RegexBuilder};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct VndbResponse<T> {
    pub results: Vec<T>,
    pub more: bool,
}

pub struct Vndb {
    client: Client,
}

impl Vndb {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    pub async fn query<T: Display, U: DeserializeOwned>(&self, endpoint: T, query: Value) -> Result<VndbResponse<U>> {
        Ok(self
            .client
            .post(format!("https://api.vndb.org/kana/{endpoint}"))
            .header("content-type", "application/json")
            .body(query.to_string())
            .send()
            .await?
            .json::<VndbResponse<U>>()
            .await?)
    }

    pub fn clean_bbcode<T: ToString>(string: T) -> String {
        Regex::new(r"https://(.+?)/")
            .unwrap()
            .replace_all(
                &RegexBuilder::new(r"\[/?[bi]\]|\[url=(.+?)\]|\[/url\]")
                    .case_insensitive(true)
                    .build()
                    .unwrap()
                    .replace_all(&string.to_string(), ""),
                "<redacted>/",
            )
            .to_string()
    }
}
