use crate::structs::{database::guilds::Guild, gateway::song_activity::SongActivity};
use std::{collections::HashMap, sync::RwLock};
use twilight_model::guild::Guild as DiscordGuild;

pub struct Cache {
    pub discord: DiscordCache,
    pub db: DatabaseCache,
    pub ordr_rendering_users: RwLock<HashMap<String, bool>>,
    pub spotify_access_token: RwLock<(String, u128)>,
}

pub struct DiscordCache {
    pub guilds: RwLock<HashMap<String, DiscordGuild>>,
    pub song_activities: RwLock<HashMap<String, SongActivity>>,
}

pub struct DatabaseCache {
    pub guilds: RwLock<HashMap<String, Guild>>,
}
