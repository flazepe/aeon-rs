use crate::{
    statics::{REDIS, colors::NOTICE_EMBED_COLOR},
    structs::{database::guilds::Guilds, simple_message::SimpleMessage},
    traits::{UserAvatarExt, UserExt},
};
use anyhow::{Error, Result};
use serde_json::from_str;
use similar::{ChangeTag, TextDiff};
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::{channel::Message as TwilightMessage, gateway::payload::incoming::MessageUpdate};

pub async fn handle(event: &MessageUpdate) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let channel_id = event.channel_id;
    let message_id = event.id;

    let Ok(old_message) = REDIS
        .get()
        .unwrap()
        .get(format!("guilds_{guild_id}_channels_{channel_id}_messages_{message_id}"))
        .await
        .and_then(|message| from_str::<TwilightMessage>(&message).map_err(Error::msg))
    else {
        return Ok(());
    };

    if old_message.content == event.content {
        return Ok(());
    }

    let old_content = SimpleMessage::from(old_message).to_string();
    let new_content = SimpleMessage::from(event.0.clone()).to_string();
    let diff = TextDiff::from_words(&old_content, &new_content)
        .iter_all_changes()
        .map(|change| {
            let string = change.as_str().unwrap_or_default().replace('`', "\\`").replace('~', "\\~").replace('*', "\\*");

            if string.trim().is_empty() {
                return string;
            }

            match change.tag() {
                ChangeTag::Equal => string,
                ChangeTag::Delete => format!("~~{string}~~"),
                ChangeTag::Insert => format!("**{string}**"),
            }
        })
        .collect::<String>();

    let embed = Embed::new()
        .set_color(NOTICE_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("Message Edited")
        .set_description(format!(
            "https://discord.com/channels/{}/{}/{}",
            event.guild_id.map(|guild_id| guild_id.to_string()).as_deref().unwrap_or_default(),
            event.channel_id,
            event.id,
        ))
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false)
        .add_field("Before", old_content.chars().take(1024).collect::<String>(), false)
        .add_field("After", diff.chars().take(1024).collect::<String>(), false)
        .set_footer(event.author.label(), Some(event.author.display_avatar_url("gif", 4096)))
        .set_timestamp(Utc::now());

    Guilds::send_log(guild_id, embed, event.author.bot).await
}
