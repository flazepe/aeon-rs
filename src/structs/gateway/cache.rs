use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};
use twilight_model::channel::Message;

pub struct Cache {
    pub channels: HashMap<String, Vec<Message>>,
    pub snipes: HashMap<String, Vec<Message>>,
    pub edit_snipes: HashMap<String, Vec<Message>>,
    pub reaction_snipes: HashMap<String, Vec<String>>,
    pub last_tio_programming_languages: HashMap<String, String>,
}

pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| {
    Mutex::new(Cache {
        channels: HashMap::new(),
        snipes: HashMap::new(),
        edit_snipes: HashMap::new(),
        reaction_snipes: HashMap::new(),
        last_tio_programming_languages: HashMap::new(),
    })
});
