pub mod anilist;
pub mod colors;
pub mod duration;
pub mod emojis;
pub mod exchange_rate;
pub mod google;
pub mod spotify;
pub mod steam;
pub mod tio;
pub mod unicode;
pub mod vndb;

use crate::structs::{config::Config, gateway::cache::Cache};
use async_once_cell::OnceCell as AsyncOnceCell;
use mongodb::Database;
use once_cell::sync::Lazy;
use std::{collections::HashMap, fs::read_to_string, sync::RwLock};
use toml::from_str;

pub static CACHE: Lazy<Cache> = Lazy::new(|| Cache {
    channels: RwLock::new(HashMap::new()),
    snipes: RwLock::new(HashMap::new()),
    edit_snipes: RwLock::new(HashMap::new()),
    reaction_snipes: RwLock::new(HashMap::new()),
    last_tio_programming_languages: RwLock::new(HashMap::new()),
});

pub static CONFIG: Lazy<Config> = Lazy::new(|| from_str(&read_to_string("config.toml").unwrap()).unwrap());

pub static MONGODB: AsyncOnceCell<Database> = AsyncOnceCell::new();
