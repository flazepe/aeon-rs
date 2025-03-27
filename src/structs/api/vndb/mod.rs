mod character;
mod character_trait;
pub mod statics;
mod tag;
mod visual_novel;

use crate::statics::{
    REQWEST,
    regex::{BBCODE_REGEX, HTTPS_URL_REGEX},
};
use anyhow::Result;
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
struct VndbResponse<T> {
    results: Vec<T>,
}

pub struct Vndb;

impl Vndb {
    async fn query<T: Display, U: DeserializeOwned>(endpoint: T, query: Value) -> Result<VndbResponse<U>> {
        Ok(REQWEST.post(format!("https://api.vndb.org/kana/{endpoint}")).json(&query).send().await?.json::<VndbResponse<U>>().await?)
    }

    pub fn clean_bbcode<T: Display>(string: T) -> String {
        HTTPS_URL_REGEX.replace_all(&BBCODE_REGEX.replace_all(&string.to_string(), ""), "<redacted>/").to_string()
    }
}
