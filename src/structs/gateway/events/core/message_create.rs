use crate::{statics::REDIS, structs::database::redis::keys::RedisKey};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle(event: &MessageCreate) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.id;

    let redis = REDIS.get().unwrap();
    let key = RedisKey::GuildChannelMessage(guild_id.to_string(), channel_id.to_string(), message_id.to_string());
    redis.set(&key, &event.0, Some(60 * 60 * 2)).await?;

    Ok(())
}
