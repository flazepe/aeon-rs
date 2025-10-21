use crate::{functions::now, statics::REDIS};
use anyhow::Result;
use serde_json::{from_str, to_string};
use twilight_model::{channel::Message as TwilightMessage, gateway::payload::incoming::MessageDeleteBulk};

pub async fn handle(event: &MessageDeleteBulk) -> Result<()> {
    let redis = REDIS.get().unwrap();

    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;

    let deleted_messages = redis
        .get_many(event.ids.iter().map(|id| format!("guilds_{guild_id}_channels_{channel_id}_messages_{id}")).collect())
        .await
        .unwrap_or_default()
        .into_iter()
        .flat_map(|string| from_str::<TwilightMessage>(&string))
        .collect::<Vec<TwilightMessage>>();

    let mut fields_values = vec![];

    for message in deleted_messages {
        let field = now();
        let Ok(value) = to_string(&message) else { continue };
        fields_values.push((field, value));
    }

    redis.hset_many(format!("guilds_{guild_id}_channels_{channel_id}_snipes"), fields_values, Some(60 * 60 * 2)).await?;

    Ok(())
}
