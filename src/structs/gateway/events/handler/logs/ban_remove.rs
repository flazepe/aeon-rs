use crate::statics::colors::SUCCESS_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::BanRemove,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &BanRemove) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let embed = Embed::new()
        .set_color(SUCCESS_COLOR)
        .unwrap_or_default()
        .set_title("User Unbanned")
        .set_description(format!("<@{}>", event.user.id))
        .add_field("Username", format!("{} ({})", event.user.name, event.user.id), false);

    Ok((event.guild_id.into(), embed))
}
