use crate::{
    functions::format_timestamp,
    statics::{MONGODB, REDIS, colors::ERROR_EMBED_COLOR},
    structs::{database::redis::keys::RedisKey, simple_message::SimpleMessage, snowflake::Snowflake},
    traits::{UserAvatarExt, UserExt},
};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::{channel::Message as TwilightMessage, gateway::payload::incoming::MessageDelete};

pub async fn handle(event: &MessageDelete) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.id;

    let snowflake = Snowflake::new(event.id)?;

    let mut embed = Embed::new()
        .set_color(ERROR_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("Message Deleted")
        .set_description(format!("https://discord.com/channels/{guild_id}/{channel_id}/{message_id}"))
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false)
        .add_field("Created", format_timestamp(snowflake.timestamp.timestamp(), true), false);

    let redis = REDIS.get().unwrap();
    let key = RedisKey::GuildChannelMessage(guild_id.to_string(), channel_id.to_string(), message_id.to_string());
    let old_message = redis.get::<TwilightMessage>(&key).await;

    if let Ok(old_message) = &old_message {
        embed = embed
            .add_field("Content", SimpleMessage::from(old_message.clone()).to_string().chars().take(1024).collect::<String>(), false)
            .set_footer(old_message.author.label(), Some(old_message.author.display_avatar_url("gif", 4096)));
    }

    embed = embed.set_timestamp(Utc::now());

    let mongodb = MONGODB.get().unwrap();
    mongodb.guilds.send_log(guild_id, embed, old_message.is_ok_and(|old_message| old_message.author.bot)).await
}
