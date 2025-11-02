use crate::{
    statics::{MONGODB, colors::ERROR_EMBED_COLOR},
    traits::EmojiReactionExt,
};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::ReactionRemoveEmoji;

pub async fn handle(event: &ReactionRemoveEmoji) -> Result<()> {
    let embed = Embed::new()
        .set_color(ERROR_EMBED_COLOR)
        .unwrap_or_default()
        .set_thumbnail(event.emoji.get_image_url())
        .set_title("Reaction Emoji Removed")
        .set_description(format!("https://discord.com/channels/{}/{}/{}", event.guild_id, event.channel_id, event.message_id))
        .add_field("Emoji", event.emoji.label(), false)
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false)
        .set_timestamp(Utc::now());

    let mongodb = MONGODB.get().unwrap();
    mongodb.guilds.send_log(event.guild_id, embed, false).await
}
