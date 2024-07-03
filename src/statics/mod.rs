pub mod colors;
pub mod emojis;
pub mod regex;

use crate::structs::{
    config::Config,
    database::{oauth::OauthToken, reminders::Reminder, tags::Tag, Collections},
    gateway::cache::Cache,
};
use mongodb::Database;
use once_cell::sync::Lazy;
use reqwest::Client;
use slashook::rest::Rest;
use std::{collections::HashMap, fs::read_to_string, sync::OnceLock, sync::RwLock};
use toml::from_str;

pub static CACHE: Lazy<Cache> = Lazy::new(|| Cache {
    channels: RwLock::new(HashMap::new()),
    snipes: RwLock::new(HashMap::new()),
    edit_snipes: RwLock::new(HashMap::new()),
    reaction_snipes: RwLock::new(HashMap::new()),
    spotify: RwLock::new(HashMap::new()),
    cooldowns: RwLock::new(HashMap::new()),
    last_tio_programming_languages: RwLock::new(HashMap::new()),
    ordr_renders: RwLock::new(HashMap::new()),
    ordr_rendering_users: RwLock::new(HashMap::new()),
    localdown_novels: RwLock::new(vec![]),
});
pub static COLLECTIONS: Lazy<Collections> = Lazy::new(|| Collections {
    oauth: MONGODB.get().unwrap().collection::<OauthToken>("oauth"),
    reminders: MONGODB.get().unwrap().collection::<Reminder>("reminders"),
    tags: MONGODB.get().unwrap().collection::<Tag>("tags"),
});
pub static CONFIG: Lazy<Config> = Lazy::new(|| from_str(&read_to_string("config.toml").unwrap()).unwrap());
pub static FLAZEPE_ID: &str = "590455379931037697";
pub static MONGODB: OnceLock<Database> = OnceLock::new();
pub static REQWEST: Lazy<Client> = Lazy::new(Client::new);
pub static REST: Lazy<Rest> = Lazy::new(|| Rest::with_token(CONFIG.bot.token.clone()));
