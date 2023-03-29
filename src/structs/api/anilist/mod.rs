mod anime;
mod components;
mod manga;

use anyhow::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

pub struct AniList {}

impl AniList {
    async fn query<T: ToString, U: DeserializeOwned>(query: T, variables: Value) -> Result<U> {
        Ok(Client::new()
            .post("https://graphql.anilist.co")
            .header("content-type", "application/json")
            .body(
                json!({
                    "query": query.to_string(),
                    "variables": variables
                })
                .to_string(),
            )
            .send()
            .await?
            .json::<U>()
            .await?)
    }
}
