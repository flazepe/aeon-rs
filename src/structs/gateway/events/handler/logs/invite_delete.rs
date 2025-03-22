use crate::statics::colors::ERROR_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::InviteDelete,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &InviteDelete) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let embed = Embed::new()
        .set_color(ERROR_COLOR)
        .unwrap_or_default()
        .set_title("Invite Deleted")
        .set_description(format!("https://discord.gg/{}", event.code))
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false);

    Ok((event.guild_id.into(), embed))
}
