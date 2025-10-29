use crate::{functions::now, statics::REDIS, structs::database::redis::keys::RedisKey};
use anyhow::Result;
use serde_json::Value;
use twilight_model::gateway::payload::incoming::MessageDelete;

pub async fn handle(event: &MessageDelete) -> Result<()> {
    let redis = REDIS.get().unwrap();

    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.id;

    if let Ok(message) =
        redis.get::<Value>(&RedisKey::GuildChannelMessage(guild_id.to_string(), channel_id.to_string(), message_id.to_string())).await
    {
        redis.hset(&RedisKey::GuildChannelSnipes(guild_id.to_string(), channel_id.to_string()), now(), message, Some(60 * 60 * 2)).await?;
    }

    Ok(())
}
