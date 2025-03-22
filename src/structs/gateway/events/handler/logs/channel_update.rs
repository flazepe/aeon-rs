use crate::statics::colors::NOTICE_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::ChannelUpdate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &ChannelUpdate) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let embed = Embed::new()
        .set_color(NOTICE_COLOR)
        .unwrap_or_default()
        .set_title("Channel Updated")
        .set_description(format!("<#{channel_id}> ({channel_id})", channel_id = event.id))
        .add_field("Name", format!("#{}", event.name.as_deref().unwrap_or("unknown")), false);

    Ok((event.guild_id, embed))
}
