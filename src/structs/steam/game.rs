use anyhow::Result;
use serde::Deserialize;
use slashook::structs::embeds::Embed;

#[derive(Deserialize)]
pub struct SteamGame {}

impl SteamGame {
    pub async fn get(game: &str, api_key: &str) -> Result<Self> {
        Ok(Self {})
    }

    pub fn format(self) -> Embed {
        Embed::new().set_description("TODO")
    }
}
