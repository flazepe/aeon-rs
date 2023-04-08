pub mod character;
pub mod visual_novel;

use anyhow::Result;
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
}
