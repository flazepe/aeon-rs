use crate::{
    functions::now,
    statics::REDIS,
    structs::{database::redis::keys::RedisKey, gateway::events::fix_embeds::EmbedFixResponse},
};
use anyhow::Result;
use serde_json::Value;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn handle(event: &MessageUpdate) -> Result<()> {
    let redis = REDIS.get().unwrap();

    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.id;

    let message_key = RedisKey::GuildChannelMessage(guild_id.to_string(), channel_id.to_string(), message_id.to_string());

    if let Ok(old_message) = redis.get::<Value>(&message_key).await {
        redis
            .hset(&RedisKey::GuildChannelEditSnipes(guild_id.to_string(), channel_id.to_string()), now(), old_message, Some(60 * 60 * 2))
            .await?;
    }

    redis.set(&message_key, &event.0, Some(60 * 60 * 2)).await?;

    let embed_fix_response_key =
        RedisKey::GuildChannelMessageEmbedFixResponse(guild_id.to_string(), channel_id.to_string(), message_id.to_string());

    if let Ok(mut embed_fix_response) = redis.get::<EmbedFixResponse>(&embed_fix_response_key).await {
        embed_fix_response.content = event.content.clone();
        redis.set(&embed_fix_response_key, embed_fix_response, Some(60 * 5)).await?;
    }

    Ok(())
}
