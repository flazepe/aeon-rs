use crate::{functions::now, statics::REDIS};
use anyhow::Result;
use serde_json::Value;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn handle(event: &MessageUpdate) -> Result<()> {
    let redis = REDIS.get().unwrap();

    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.id;

    let key = format!("guilds_{guild_id}_channels_{channel_id}_messages_{message_id}");

    if let Ok(old_message) = redis.get::<Value>(&key).await {
        redis.hset(format!("guilds_{guild_id}_channels_{channel_id}_edit-snipes"), now(), old_message, Some(60 * 60 * 2)).await?;
    }

    redis.set(key, &event.0, Some(60 * 60 * 2)).await?;

    Ok(())
}
