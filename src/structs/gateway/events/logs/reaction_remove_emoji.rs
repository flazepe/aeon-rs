use crate::{statics::colors::ERROR_EMBED_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::{channel::message::EmojiReactionType, gateway::payload::incoming::ReactionRemoveEmoji};

pub async fn handle(event: &ReactionRemoveEmoji) -> Result<()> {
    let embed = Embed::new()
        .set_color(ERROR_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("Reaction Emoji Removed")
        .set_description(format!("https://discord.com/channels/{}/{}/{}", event.guild_id, event.channel_id, event.message_id))
        .add_field(
            "Emoji",
            match &event.emoji {
                EmojiReactionType::Custom { name, id, .. } => {
                    format!("[{}](https://cdn.discordapp.com/emojis/{id})", name.as_deref().unwrap_or("<unknown>"))
                },
                EmojiReactionType::Unicode { name } => name.clone(),
            },
            false,
        )
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false)
        .set_timestamp(Utc::now());

    Guilds::send_log(event.guild_id, embed, false).await
}
