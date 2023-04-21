use std::collections::HashMap;
use std::sync::RwLock;
use twilight_model::channel::Message;

pub struct Cache {
    pub channels: RwLock<HashMap<String, Vec<Message>>>,
    pub snipes: RwLock<HashMap<String, Vec<Message>>>,
    pub edit_snipes: RwLock<HashMap<String, Vec<Message>>>,
    pub reaction_snipes: RwLock<HashMap<String, Vec<String>>>,
    pub cooldowns: RwLock<HashMap<String, u64>>,
    pub last_tio_programming_languages: RwLock<HashMap<String, String>>,
}
