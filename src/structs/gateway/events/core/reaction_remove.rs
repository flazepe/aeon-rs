use crate::{functions::now, statics::REDIS, structs::database::redis::keys::RedisKey};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::ReactionRemove;

pub async fn handle(event: &ReactionRemove) -> Result<()> {
    let redis = REDIS.get().unwrap();

    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.message_id;

    redis
        .hset(
            &RedisKey::GuildChannelMessageReactionSnipes(guild_id.to_string(), channel_id.to_string(), message_id.to_string()),
            now(),
            &event.0,
            Some(60 * 30),
        )
        .await?;

    Ok(())
}
