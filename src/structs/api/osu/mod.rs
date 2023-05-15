mod user;

use crate::{
    statics::{CONFIG, REQWEST},
    structs::database::oauth::OAuth,
};
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub struct Osu {}

impl Osu {
    pub async fn query<T: Display, U: DeserializeOwned>(endpoint: T) -> Result<U> {
        Ok(REQWEST
            .get(format!("https://osu.ppy.sh/api/v2/{endpoint}"))
            .header(
                "authorization",
                OAuth::new(
                    "osu",
                    REQWEST.post("https://osu.ppy.sh/oauth/token").header("content-type", "application/x-www-form-urlencoded").body(
                        format!(
                            "client_id={}&client_secret={}&grant_type=client_credentials&scope=public",
                            CONFIG.api.osu.client_id, CONFIG.api.osu.client_secret,
                        ),
                    ),
                )
                .get_token()
                .await?,
            )
            .send()
            .await?
            .json::<U>()
            .await?)
    }
}
