mod user;

use crate::{
    statics::{CONFIG, REQWEST},
    structs::database::oauth::Oauth,
};
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub struct Osu;

impl Osu {
    pub async fn query<T: Display, U: DeserializeOwned>(endpoint: T) -> Result<U> {
        Ok(REQWEST
            .get(format!("https://osu.ppy.sh/api/v2/{endpoint}"))
            .header(
                "authorization",
                Oauth::new(
                    "osu",
                    REQWEST.post("https://osu.ppy.sh/oauth/token").form(&[
                        ("client_id", CONFIG.api.osu.client_id.as_str()),
                        ("client_secret", CONFIG.api.osu.client_secret.as_str()),
                        ("grant_type", "client_credentials"),
                        ("scope", "public"),
                    ]),
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
