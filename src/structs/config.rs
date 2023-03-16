use anyhow::Result;
use serde::Deserialize;
use std::fs::read_to_string;
use toml::from_str;

#[derive(Deserialize)]
pub struct Config {
    pub bot: BotConfig,
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub client_id: String,
    pub guild_id: Option<String>,
    pub public_key: String,
    pub token: String,
}

impl Config {
    pub fn load() -> Result<Config> {
        Ok(from_str(&read_to_string("config.toml")?)?)
    }
}
