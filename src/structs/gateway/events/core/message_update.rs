use crate::{
    functions::now,
    structs::database::{Database, redis::keys::RedisKey},
};
use anyhow::Result;
use serde_json::Value;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn handle(event: &MessageUpdate) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.id;

    let redis = Database::get_redis()?;
    let message_key = RedisKey::GuildChannelMessage(guild_id.to_string(), channel_id.to_string(), message_id.to_string());
    let snipes_key = RedisKey::GuildChannelEditSnipes(guild_id.to_string(), channel_id.to_string());

    if let Ok(old_message) = redis.get::<Value>(&message_key).await {
        redis.hset(&snipes_key, now(), old_message, Some(60 * 60 * 2)).await?;
    }

    redis.set(&message_key, &event.0, Some(60 * 60 * 2)).await?;

    Ok(())
}
