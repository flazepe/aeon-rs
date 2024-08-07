use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bot: BotConfig,
    pub database: DatabaseConfig,
    pub api: APIConfig,
}

#[derive(Deserialize, Debug)]
pub struct BotConfig {
    pub client_id: String,
    pub guild_id: Option<String>,
    pub public_key: String,
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub mongodb_uri: String,
}

#[derive(Deserialize, Debug)]
pub struct APIConfig {
    pub google_assistant: GoogleAssistantConfig,
    pub ordr_key: String,
    pub osu: OsuConfig,
    pub saucenao_key: String,
    pub spotify_dc: String,
    pub spotify_token: String,
    pub steam_key: String,
    pub virtualearth_key: String,
    pub waaai_key: String,
}

#[derive(Deserialize, Debug)]
pub struct GoogleAssistantConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub device_id: String,
    pub device_model_id: String,
}

#[derive(Deserialize, Debug)]
pub struct OsuConfig {
    pub client_id: String,
    pub client_secret: String,
}
