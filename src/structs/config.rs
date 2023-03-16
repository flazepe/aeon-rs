use serde::Deserialize;

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
