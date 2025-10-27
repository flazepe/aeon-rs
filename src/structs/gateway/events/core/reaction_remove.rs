use crate::{functions::now, statics::REDIS};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::ReactionRemove;

pub async fn handle(event: &ReactionRemove) -> Result<()> {
    let redis = REDIS.get().unwrap();

    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.message_id;

    redis
        .hset(format!("guilds_{guild_id}_channels_{channel_id}_messages_{message_id}_reaction-snipes"), now(), &event.0, Some(60 * 30))
        .await?;

    Ok(())
}
