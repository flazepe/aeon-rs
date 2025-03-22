use crate::statics::colors::SUCCESS_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::ChannelCreate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &ChannelCreate) -> Result<(Option<Id<GuildMarker>>, Option<Embed>)> {
    let embed = Embed::new()
        .set_color(SUCCESS_COLOR)
        .unwrap_or_default()
        .set_title("Channel Created")
        .set_description(format!("<#{channel_id}> ({channel_id})", channel_id = event.id))
        .add_field("Name", format!("#{}", event.name.as_deref().unwrap_or("unknown")), false);

    Ok((event.guild_id, embed.into()))
}
