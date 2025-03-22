use crate::statics::colors::NOTICE_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::GuildEmojisUpdate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &GuildEmojisUpdate) -> Result<(Option<Id<GuildMarker>>, Option<Embed>)> {
    let embed = Embed::new().set_color(NOTICE_COLOR).unwrap_or_default().set_title("Server Emojis Updated");

    Ok((event.guild_id.into(), embed.into()))
}
