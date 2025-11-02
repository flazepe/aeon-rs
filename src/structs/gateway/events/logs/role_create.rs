use crate::statics::{MONGODB, colors::SUCCESS_EMBED_COLOR};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::RoleCreate;

pub async fn handle(event: &RoleCreate) -> Result<()> {
    let embed = Embed::new()
        .set_color(SUCCESS_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("Role Created")
        .set_description(format!("<@&{role_id}> ({role_id})", role_id = event.role.id))
        .add_field("Name", format!("@{}", event.role.name), false)
        .set_timestamp(Utc::now());

    let mongodb = MONGODB.get().unwrap();
    mongodb.guilds.send_log(event.guild_id, embed, false).await
}
