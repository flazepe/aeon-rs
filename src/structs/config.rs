use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub bot: BotConfig,
    pub database: DatabaseConfig,
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
pub struct DatabaseConfig {
    pub mongodb_uri: String,
}

#[derive(Deserialize)]
pub struct APIConfig {
    pub saucenao_key: String,
    pub steam_key: String,
    pub virtualearth_key: String,
}
