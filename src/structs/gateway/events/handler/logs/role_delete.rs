use crate::{statics::colors::ERROR_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::gateway::payload::incoming::RoleDelete;

pub async fn log(event: &RoleDelete) -> Result<()> {
    let embed = Embed::new()
        .set_color(ERROR_COLOR)
        .unwrap_or_default()
        .set_title("Role Deleted")
        .set_description(format!("<@&{role_id}> ({role_id})", role_id = event.role_id));

    Guilds::send_log(event.guild_id, embed).await
}
