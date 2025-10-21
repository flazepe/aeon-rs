use crate::{
    functions::format_timestamp,
    statics::{REDIS, colors::ERROR_EMBED_COLOR},
    structs::{database::guilds::Guilds, simple_message::SimpleMessage, snowflake::Snowflake},
    traits::{UserAvatarExt, UserExt},
};
use anyhow::{Error, Result};
use serde_json::from_str;
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

    let old_message = REDIS
        .get()
        .unwrap()
        .get(format!("guilds_{guild_id}_channels_{channel_id}_messages_{message_id}"))
        .await
        .and_then(|message| from_str::<TwilightMessage>(&message).map_err(Error::msg));

    if let Ok(old_message) = &old_message {
        embed = embed
            .add_field("Content", SimpleMessage::from(old_message.clone()).to_string().chars().take(1024).collect::<String>(), false)
            .set_footer(old_message.author.label(), Some(old_message.author.display_avatar_url("gif", 4096)));
    }

    embed = embed.set_timestamp(Utc::now());

    Guilds::send_log(guild_id, embed, old_message.is_ok_and(|old_message| old_message.author.bot)).await
}
