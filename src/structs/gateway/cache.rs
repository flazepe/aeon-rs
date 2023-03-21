use std::collections::HashMap;
use twilight_model::channel::Message;

pub struct Cache {
    pub channels: HashMap<String, Vec<Message>>,
    pub snipes: HashMap<String, Vec<Message>>,
    pub edit_snipes: HashMap<String, Vec<Message>>,
    pub reaction_snipes: HashMap<String, Vec<String>>,
    pub last_tio_programming_languages: HashMap<String, String>,
}
