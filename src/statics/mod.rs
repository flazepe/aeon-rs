pub mod colors;
pub mod emojis;
pub mod regex;

use crate::structs::{
    cache::{Cache, DatabaseCache, DiscordCache},
    config::Config,
    database::{Collections, guilds::Guild, oauth::OauthToken, reminders::Reminder, tags::Tag},
};
use mongodb::Database;
use reqwest::Client;
use slashook::rest::Rest;
use std::{
    collections::HashMap,
    fs::read_to_string,
    sync::{LazyLock, OnceLock, RwLock},
};
use toml::from_str;

pub static CACHE: LazyLock<Cache> = LazyLock::new(|| Cache {
    discord: DiscordCache {
        guilds: RwLock::new(HashMap::new()),
        channels: RwLock::new(HashMap::new()),
        snipes: RwLock::new(HashMap::new()),
        edit_snipes: RwLock::new(HashMap::new()),
        reaction_snipes: RwLock::new(HashMap::new()),
        song_activities: RwLock::new(HashMap::new()),
        command_responses: RwLock::new(HashMap::new()),
        embed_fix_responses: RwLock::new(HashMap::new()),
    },
    db: DatabaseCache { guilds: RwLock::new(HashMap::new()) },
    cooldowns: RwLock::new(HashMap::new()),
    last_piston_programming_languages: RwLock::new(HashMap::new()),
    last_tio_programming_languages: RwLock::new(HashMap::new()),
    ordr_rendering_users: RwLock::new(HashMap::new()),
    spotify_access_token: RwLock::new(Default::default()),
});
pub static COLLECTIONS: LazyLock<Collections> = LazyLock::new(|| Collections {
    guilds: MONGODB.get().unwrap().collection::<Guild>("guilds"),
    oauth: MONGODB.get().unwrap().collection::<OauthToken>("oauth"),
    reminders: MONGODB.get().unwrap().collection::<Reminder>("reminders"),
    tags: MONGODB.get().unwrap().collection::<Tag>("tags"),
});
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| from_str(&read_to_string("config.toml").unwrap()).unwrap());
pub static DEFAULT_PREFIXES: LazyLock<[String; 2]> = LazyLock::new(|| [format!("<@{}>", CONFIG.bot.client_id), "aeon".into()]);
pub static FLAZEPE_ID: &str = "590455379931037697";
pub static MONGODB: OnceLock<Database> = OnceLock::new();
pub static REQWEST: LazyLock<Client> = LazyLock::new(Client::new);
pub static REST: LazyLock<Rest> = LazyLock::new(|| Rest::with_token(CONFIG.bot.token.clone()));
