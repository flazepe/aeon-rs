use crate::statics::{CACHE, COLLECTIONS, REST};
use anyhow::Result;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use slashook::{commands::MessageResponse, structs::messages::Message};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Guild {
    pub _id: String,
    pub fix_embeds: GuildFixEmbeds,
    pub logs: GuildLogs,
    pub prefixes: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct GuildFixEmbeds {
    pub enabled: bool,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct GuildLogs {
    pub enabled: bool,
    pub channel_id: Option<String>,
    pub ignore_bots: bool,
}

pub struct Guilds;

impl Guilds {
    pub async fn get<T: Display>(guild_id: T) -> Result<Guild> {
        let guild_id = guild_id.to_string();

        if let Some(guild) = CACHE.guilds.read().unwrap().get(&guild_id) {
            return Ok(guild.clone());
        }

        let guild = match COLLECTIONS.guilds.find_one(doc! { "_id": &guild_id }).await? {
            Some(guild) => guild,
            None => {
                let guild = Guild { _id: guild_id.clone(), ..Default::default() };
                COLLECTIONS.guilds.insert_one(&guild).await?;
                guild
            },
        };

        CACHE.guilds.write().unwrap().insert(guild_id.clone(), guild.clone());
        Ok(guild)
    }

    pub async fn update(guild: Guild) -> Result<()> {
        CACHE.guilds.write().unwrap().insert(guild._id.clone(), guild.clone());
        COLLECTIONS.guilds.replace_one(doc! { "_id": &guild._id }, guild).await?;
        Ok(())
    }

    pub async fn delete<T: Display>(guild_id: T) -> Result<()> {
        let guild_id = guild_id.to_string();
        CACHE.guilds.write().unwrap().remove(&guild_id);
        COLLECTIONS.guilds.delete_one(doc! { "_id": guild_id }).await?;
        Ok(())
    }

    pub async fn send_log<T: Display, U: Into<MessageResponse>>(guild_id: T, response: U, is_bot: bool) -> Result<()> {
        let guild = Self::get(guild_id).await?;

        if !guild.logs.enabled || (guild.logs.ignore_bots && is_bot) {
            return Ok(());
        }

        let Some(logs_channel_id) = &guild.logs.channel_id else { return Ok(()) };
        _ = Message::create(&REST, logs_channel_id, response).await;

        Ok(())
    }
}
