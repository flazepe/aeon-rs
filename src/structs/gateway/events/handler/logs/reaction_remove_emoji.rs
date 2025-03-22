use crate::statics::colors::ERROR_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    channel::message::EmojiReactionType,
    gateway::payload::incoming::ReactionRemoveEmoji,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &ReactionRemoveEmoji) -> Result<(Option<Id<GuildMarker>>, Option<Embed>)> {
    let embed = Embed::new()
        .set_color(ERROR_COLOR)
        .unwrap_or_default()
        .set_title("Reaction Emoji Removed")
        .set_description(format!("https://discord.com/channels/{}/{}/{}", event.guild_id, event.channel_id, event.message_id))
        .add_field(
            "Emoji",
            match &event.emoji {
                EmojiReactionType::Custom { name, id, animated: _ } => {
                    format!("[{}](https://cdn.discordapp.com/emojis/{id})", name.as_deref().unwrap_or("<unknown>"))
                },
                EmojiReactionType::Unicode { name } => name.clone(),
            },
            false,
        )
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false);

    Ok((event.guild_id.into(), embed.into()))
}
