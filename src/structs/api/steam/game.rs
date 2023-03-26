use crate::statics::colors::PRIMARY_COLOR;
use anyhow::Result;
use serde::Deserialize;
use slashook::structs::embeds::Embed;

#[derive(Deserialize)]
pub struct SteamGame {}

impl SteamGame {
    pub async fn get<T: ToString>(game: T) -> Result<Self> {
        Ok(Self {})
    }

    pub fn format(self) -> Embed {
        Embed::new()
            .set_color(PRIMARY_COLOR)
            .unwrap_or_default()
            .set_description("TODO")
    }
}
