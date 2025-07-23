use crate::structs::{database::guilds::Guild, gateway::song_activity::SongActivity};
use slashook::structs::messages::Message as SlashookMessage;
use std::{collections::HashMap, sync::RwLock};
use twilight_model::channel::Message;

pub struct Cache {
    pub guilds: RwLock<HashMap<String, Guild>>,
    pub channels: RwLock<HashMap<String, Vec<Message>>>,
    pub snipes: RwLock<HashMap<String, Vec<Message>>>,
    pub edit_snipes: RwLock<HashMap<String, Vec<Message>>>,
    pub reaction_snipes: RwLock<HashMap<String, Vec<String>>>,
    pub song_activities: RwLock<HashMap<String, SongActivity>>,
    pub cooldowns: RwLock<HashMap<String, u64>>,
    pub command_responses: RwLock<HashMap<String, SlashookMessage>>,
    pub embed_fix_responses: RwLock<HashMap<String, SlashookMessage>>,
    pub last_piston_programming_languages: RwLock<HashMap<String, String>>,
    pub last_tio_programming_languages: RwLock<HashMap<String, String>>,
    pub ordr_renders: RwLock<HashMap<u64, String>>,
    pub ordr_rendering_users: RwLock<HashMap<String, bool>>,
    pub spotify_access_token: RwLock<(String, u128)>,
}
