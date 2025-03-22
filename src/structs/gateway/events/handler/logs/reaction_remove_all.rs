use crate::statics::colors::ERROR_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::ReactionRemoveAll,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &ReactionRemoveAll) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let embed = Embed::new()
        .set_color(ERROR_COLOR)
        .unwrap_or_default()
        .set_title("All Reactions Removed")
        .set_description(format!(
            "https://discord.com/channels/{}/{}/{}",
            event.guild_id.map(|guild_id| guild_id.to_string()).as_deref().unwrap_or(""),
            event.channel_id,
            event.message_id,
        ))
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false);

    Ok((event.guild_id, embed))
}
