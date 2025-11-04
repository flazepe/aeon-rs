mod user;

use crate::{statics::REQWEST, structs::database::Database};
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub struct Osu;

impl Osu {
    pub async fn query<T: Display, U: DeserializeOwned>(endpoint: T) -> Result<U> {
        let mongodb = Database::get_mongodb()?;

        Ok(REQWEST
            .get(format!("https://osu.ppy.sh/api/v2/{endpoint}"))
            .header("authorization", mongodb.oauth.osu.get_token().await?)
            .send()
            .await?
            .json::<U>()
            .await?)
    }
}
