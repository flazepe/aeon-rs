use crate::statics::{CACHE, COLLECTIONS};
use anyhow::Result;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Guild {
    pub _id: String,
    pub fix_embeds: bool,
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
}
