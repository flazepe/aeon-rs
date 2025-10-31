use crate::{functions::now, statics::REDIS, structs::database::redis::keys::RedisKey};
use anyhow::Result;
use serde_json::to_string;
use twilight_model::{channel::Message as TwilightMessage, gateway::payload::incoming::MessageDeleteBulk};

pub async fn handle(event: &MessageDeleteBulk) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;

    let redis = REDIS.get().unwrap();
    let deleted_messages = redis
        .get_many::<TwilightMessage>(
            event
                .ids
                .iter()
                .map(|id| RedisKey::GuildChannelMessage(guild_id.to_string(), channel_id.to_string(), id.to_string()))
                .collect(),
        )
        .await
        .unwrap_or_default();

    let mut fields_values = vec![];

    for message in deleted_messages {
        let field = now();
        let Ok(value) = to_string(&message) else { continue };
        fields_values.push((field, value));
    }

    let key = RedisKey::GuildChannelSnipes(guild_id.to_string(), channel_id.to_string());
    redis.hset_many(&key, fields_values, Some(60 * 60 * 2)).await?;

    Ok(())
}
