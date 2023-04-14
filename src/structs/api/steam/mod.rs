pub mod country;
pub mod game;
pub mod user;
pub mod user_bans;
pub mod user_vanity;

use crate::statics::CONFIG;
use anyhow::Result;
use reqwest::get;
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub struct Steam {}

impl Steam {
    pub async fn query<T: Display, U: Display, V: DeserializeOwned>(endpoint: T, query: U) -> Result<V> {
        Ok(get(format!(
            "http://api.steampowered.com/ISteamUser/{endpoint}?key={}&{query}",
            CONFIG.api.steam_key
        ))
        .await?
        .json::<V>()
        .await?)
    }
}
