pub mod colors;
pub mod currencies;
pub mod dns_codes;
pub mod duration;
pub mod emojis;
pub mod google_translate_languages;
pub mod steam;
pub mod tio_programming_languages;
pub mod unicode;

use crate::structs::{config::*, gateway::cache::Cache};
use async_once_cell::OnceCell as AsyncOnceCell;
use mongodb::Database;
use once_cell::sync::Lazy;
use std::{collections::HashMap, fs::read_to_string, sync::Mutex};
use toml::from_str;

pub static CACHE: Lazy<Cache> = Lazy::new(|| Cache {
    channels: Mutex::new(HashMap::new()),
    snipes: Mutex::new(HashMap::new()),
    edit_snipes: Mutex::new(HashMap::new()),
    reaction_snipes: Mutex::new(HashMap::new()),
    last_tio_programming_languages: Mutex::new(HashMap::new()),
});

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| from_str(&read_to_string("config.toml").unwrap()).unwrap());

pub static MONGODB: AsyncOnceCell<Database> = AsyncOnceCell::new();
