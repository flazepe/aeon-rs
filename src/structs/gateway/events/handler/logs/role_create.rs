use crate::statics::colors::SUCCESS_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::RoleCreate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &RoleCreate) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let embed = Embed::new()
        .set_color(SUCCESS_COLOR)
        .unwrap_or_default()
        .set_title("Role Created")
        .set_description(format!("<@&{role_id}> ({role_id})", role_id = event.role.id))
        .add_field("Name", format!("@{}", event.role.name), false);

    Ok((event.guild_id.into(), embed))
}
