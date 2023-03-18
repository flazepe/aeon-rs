use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs::read_to_string;
use toml::from_str;

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| from_str(&read_to_string("config.toml").unwrap()).unwrap());

#[derive(Deserialize)]
pub struct Config {
    pub bot: BotConfig,
    pub api: APIConfig,
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub client_id: String,
    pub guild_id: Option<String>,
    pub public_key: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct APIConfig {
    pub saucenao_key: String,
    pub steam_key: String,
}
