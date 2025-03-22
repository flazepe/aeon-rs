use crate::statics::colors::NOTICE_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::ChannelPinsUpdate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &ChannelPinsUpdate) -> Result<(Option<Id<GuildMarker>>, Option<Embed>)> {
    let embed = Embed::new()
        .set_color(NOTICE_COLOR)
        .unwrap_or_default()
        .set_title("Channel Pins Updated")
        .set_description(format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id));

    Ok((event.guild_id, embed.into()))
}
