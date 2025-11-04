use crate::{
    functions::now,
    structs::database::{Database, redis::keys::RedisKey},
};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::ReactionRemove;

pub async fn handle(event: &ReactionRemove) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;

    let redis = Database::get_redis()?;
    let key = RedisKey::GuildChannelReactionSnipes(guild_id.to_string(), channel_id.to_string());
    redis.hset(&key, now(), &event.0, Some(60 * 30)).await?;

    Ok(())
}
