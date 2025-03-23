use crate::{statics::colors::SUCCESS_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::gateway::payload::incoming::RoleCreate;

pub async fn log(event: &RoleCreate) -> Result<()> {
    let embed = Embed::new()
        .set_color(SUCCESS_COLOR)
        .unwrap_or_default()
        .set_title("Role Created")
        .set_description(format!("<@&{role_id}> ({role_id})", role_id = event.role.id))
        .add_field("Name", format!("@{}", event.role.name), false);

    Guilds::send_log(event.guild_id, embed).await
}
