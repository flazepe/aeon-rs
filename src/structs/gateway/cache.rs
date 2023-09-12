use crate::structs::{api::localdown::LocalDownNovel, gateway::song_activity::SongActivity};
use std::{collections::HashMap, sync::RwLock};
use twilight_model::channel::Message;

pub struct Cache {
    pub channels: RwLock<HashMap<String, Vec<Message>>>,
    pub snipes: RwLock<HashMap<String, Vec<Message>>>,
    pub edit_snipes: RwLock<HashMap<String, Vec<Message>>>,
    pub reaction_snipes: RwLock<HashMap<String, Vec<String>>>,
    pub spotify: RwLock<HashMap<String, SongActivity>>,
    pub cooldowns: RwLock<HashMap<String, u64>>,
    pub last_tio_programming_languages: RwLock<HashMap<String, String>>,
    pub ordr_renders: RwLock<HashMap<u64, String>>,
    pub ordr_rendering_users: RwLock<HashMap<String, bool>>,
    pub localdown_novels: RwLock<Vec<LocalDownNovel>>,
}
