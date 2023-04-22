pub mod country;
pub mod game;
pub mod user;
pub mod user_bans;
pub mod user_vanity;

use crate::statics::CONFIG;
use anyhow::Result;
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;

pub struct Steam {}

impl Steam {
    pub async fn query<T: Display, U: Serialize + ?Sized, V: DeserializeOwned>(endpoint: T, query: &U) -> Result<V> {
        Ok(Client::new()
            .get(format!("http://api.steampowered.com/ISteamUser/{endpoint}"))
            .query(&[("key", CONFIG.api.steam_key.as_str())])
            .query(&query)
            .send()
            .await?
            .json::<V>()
            .await?)
    }
}
