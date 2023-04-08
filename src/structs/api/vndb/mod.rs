pub mod character;
pub mod visual_novel;

use anyhow::Result;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{json, Value};
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

    pub async fn query<T: Display, U: Display, V: DeserializeOwned>(
        &self,
        endpoint: T,
        filters: Value,
        fields: U,
    ) -> Result<VndbResponse<V>> {
        Ok(self
            .client
            .post(format!("https://api.vndb.org/kana/{endpoint}"))
            .header("content-type", "application/json")
            .body(
                json!({
                    "filters": filters,
                    "fields": fields.to_string()
                })
                .to_string(),
            )
            .send()
            .await?
            .json::<VndbResponse<V>>()
            .await?)
    }
}
