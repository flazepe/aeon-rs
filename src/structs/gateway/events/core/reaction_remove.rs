use crate::{functions::now, statics::REDIS, structs::database::redis::keys::RedisKey};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::ReactionRemove;

pub async fn handle(event: &ReactionRemove) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.message_id;

    let redis = REDIS.get().unwrap();
    let key = RedisKey::GuildChannelMessageReactionSnipes(guild_id.to_string(), channel_id.to_string(), message_id.to_string());
    redis.hset(&key, now(), &event.0, Some(60 * 30)).await?;

    Ok(())
}
