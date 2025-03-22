use crate::statics::colors::NOTICE_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::RoleUpdate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &RoleUpdate) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let embed = Embed::new()
        .set_color(NOTICE_COLOR)
        .unwrap_or_default()
        .set_title("Role Updated")
        .set_description(format!("<@&{role_id}> ({role_id})", role_id = event.role.id))
        .add_field("Name", format!("@{}", event.role.name), false);

    Ok((event.guild_id.into(), embed))
}
