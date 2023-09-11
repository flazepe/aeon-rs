use crate::structs::api::localdown::LocalDownNovel;
use serde::Serialize;
use std::{collections::HashMap, sync::RwLock};
use twilight_model::channel::Message;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SongActivity {
    pub service: SongActivityService,
    pub style: SongActivityStyle,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_cover: String,
    pub timestamps: (u64, u64),
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SongActivityService {
    Aeon,
    Deezer,
    Itunes,
    SoundCloud,
    Spotify,
    YouTube,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SongActivityStyle {
    Classic,
    Nori,
    Rovi,
    Vxc,
}

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
