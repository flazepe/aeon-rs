use crate::statics::colors::NOTICE_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::GuildUpdate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &GuildUpdate) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let embed = Embed::new().set_color(NOTICE_COLOR).unwrap_or_default().set_title("Server Updated");

    Ok((event.id.into(), embed))
}
