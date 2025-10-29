use crate::{statics::REDIS, structs::database::redis::keys::RedisKey};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle(event: &MessageCreate) -> Result<()> {
    let redis = REDIS.get().unwrap();

    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.id;

    redis
        .set(
            &RedisKey::GuildChannelMessage(guild_id.to_string(), channel_id.to_string(), message_id.to_string()),
            &event.0,
            Some(60 * 60 * 2),
        )
        .await?;

    Ok(())
}
