use crate::{statics::colors::NOTICE_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::RoleUpdate;

pub async fn handle(event: &RoleUpdate) -> Result<()> {
    let embed = Embed::new()
        .set_color(NOTICE_COLOR)
        .unwrap_or_default()
        .set_title("Role Updated")
        .set_description(format!("<@&{role_id}> ({role_id})", role_id = event.role.id))
        .add_field("Name", format!("@{}", event.role.name), false)
        .set_timestamp(Utc::now());

    Guilds::send_log(event.guild_id, embed).await
}
