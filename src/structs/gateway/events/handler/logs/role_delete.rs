use crate::statics::colors::ERROR_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::RoleDelete,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &RoleDelete) -> Result<(Option<Id<GuildMarker>>, Option<Embed>)> {
    let embed = Embed::new()
        .set_color(ERROR_COLOR)
        .unwrap_or_default()
        .set_title("Role Deleted")
        .set_description(format!("<@&{role_id}> ({role_id})", role_id = event.role_id));

    Ok((event.guild_id.into(), embed.into()))
}
