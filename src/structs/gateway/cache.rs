use std::collections::HashMap;
use std::sync::Mutex;
use twilight_model::channel::Message;

pub struct Cache {
    pub channels: Mutex<HashMap<String, Vec<Message>>>,
    pub snipes: Mutex<HashMap<String, Vec<Message>>>,
    pub edit_snipes: Mutex<HashMap<String, Vec<Message>>>,
    pub reaction_snipes: Mutex<HashMap<String, Vec<String>>>,
    pub last_tio_programming_languages: Mutex<HashMap<String, String>>,
}
