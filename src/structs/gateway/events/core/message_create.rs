use crate::statics::REDIS;
use anyhow::Result;
use serde_json::to_string;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle(event: &MessageCreate) -> Result<()> {
    let redis = REDIS.get().unwrap();

    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.id;

    redis.set(format!("guilds_{guild_id}_channels_{channel_id}_messages_{message_id}"), to_string(&event.0)?, Some(60 * 60 * 2)).await?;

    Ok(())
}
