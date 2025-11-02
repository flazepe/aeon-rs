use crate::{
    macros::yes_no,
    statics::{MONGODB, colors::ERROR_EMBED_COLOR},
    traits::EmojiReactionExt,
};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::ReactionRemove;

pub async fn handle(event: &ReactionRemove) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };

    let embed = Embed::new()
        .set_color(ERROR_EMBED_COLOR)
        .unwrap_or_default()
        .set_thumbnail(event.emoji.get_image_url())
        .set_title("Reaction Removed")
        .set_description(format!(
            "https://discord.com/channels/{}/{}/{}",
            event.guild_id.map(|guild_id| guild_id.to_string()).as_deref().unwrap_or_default(),
            event.channel_id,
            event.message_id,
        ))
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false)
        .add_field("Emoji", event.emoji.label(), false)
        .add_field("Member", format!("<@{user_id}> ({user_id})", user_id = event.user_id), false)
        .add_field(
            "Burst",
            format!(
                "{}{}",
                yes_no!(event.burst),
                if event.burst_colors.is_empty() {
                    "".into()
                } else {
                    format!(" ({})", event.burst_colors.iter().map(|color| format!("`{color}`")).collect::<Vec<String>>().join(", "))
                },
            ),
            false,
        )
        .set_timestamp(Utc::now());

    let mongodb = MONGODB.get().unwrap();
    mongodb.guilds.send_log(guild_id, embed, false).await
}
