use crate::statics::colors::NOTICE_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::GuildStickersUpdate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &GuildStickersUpdate) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let embed = Embed::new().set_color(NOTICE_COLOR).unwrap_or_default().set_title("Server Stickers Updated");

    Ok((event.guild_id.into(), embed))
}
