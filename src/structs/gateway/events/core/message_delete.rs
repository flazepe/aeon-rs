use crate::{functions::now, statics::REDIS};
use anyhow::Result;
use serde_json::Value;
use twilight_model::gateway::payload::incoming::MessageDelete;

pub async fn handle(event: &MessageDelete) -> Result<()> {
    let redis = REDIS.get().unwrap();

    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.id;

    if let Ok(message) = redis.get::<Value>(format!("guilds_{guild_id}_channels_{channel_id}_messages_{message_id}")).await {
        redis.hset(format!("guilds_{guild_id}_channels_{channel_id}_snipes"), now(), message, Some(60 * 60 * 2)).await?;
    }

    Ok(())
}
