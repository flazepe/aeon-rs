mod country;
mod game;
pub mod statics;
mod user;
mod user_bans;
mod user_vanity;

use crate::statics::{CONFIG, REQWEST};
use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Display;

pub struct Steam;

impl Steam {
    pub async fn query<T: Display, U: Serialize + ?Sized, V: DeserializeOwned>(endpoint: T, query: &U) -> Result<V> {
        Ok(REQWEST
            .get(format!("http://api.steampowered.com/ISteamUser/{endpoint}"))
            .query(&[("key", &CONFIG.api.steam_key)])
            .query(&query)
            .send()
            .await?
            .json::<V>()
            .await?)
    }
}
