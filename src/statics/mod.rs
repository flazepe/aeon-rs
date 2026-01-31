pub mod colors;
pub mod regex;

use crate::structs::{
    cache::{Cache, DatabaseCache, DiscordCache},
    config::Config,
    database::{mongodb::MongoDB, redis::Redis},
    emoji_manager::EmojiManager,
};
use reqwest::Client as ReqwestClient;
use slashook::rest::Rest;
use std::{
    collections::HashMap,
    fs::read_to_string,
    sync::{LazyLock, OnceLock, RwLock},
};
use toml::from_str;

pub static CACHE: LazyLock<Cache> = LazyLock::new(|| Cache {
    discord: DiscordCache { guilds: RwLock::new(HashMap::new()), song_activities: RwLock::new(HashMap::new()) },
    db: DatabaseCache { guilds: RwLock::new(HashMap::new()) },
    ordr_rendering_users: RwLock::new(HashMap::new()),
    spotify_access_token: RwLock::new(Default::default()),
});
pub static CONFIG_PATH: &str = "config.toml";
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| from_str(&read_to_string(CONFIG_PATH).unwrap()).unwrap());
pub static DEFAULT_PREFIXES: LazyLock<[String; 2]> = LazyLock::new(|| [format!("<@{}>", CONFIG.bot.client_id), "aeon".into()]);
pub static EMOJIS: OnceLock<EmojiManager> = OnceLock::new();
pub static FLAZEPE_ID: &str = "590455379931037697";
pub static MONGODB: OnceLock<MongoDB> = OnceLock::new();
pub static REDIS: OnceLock<Redis> = OnceLock::new();
pub static REQWEST: LazyLock<ReqwestClient> = LazyLock::new(ReqwestClient::new);
pub static REST: LazyLock<Rest> = LazyLock::new(|| Rest::with_token(CONFIG.bot.token.clone()));
