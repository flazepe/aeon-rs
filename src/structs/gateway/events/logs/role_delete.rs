use crate::{statics::colors::ERROR_EMBED_COLOR, structs::database::Database};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::RoleDelete;

pub async fn handle(event: &RoleDelete) -> Result<()> {
    let embed = Embed::new()
        .set_color(ERROR_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("Role Deleted")
        .set_description(format!("<@&{role_id}> ({role_id})", role_id = event.role_id))
        .set_timestamp(Utc::now());

    let mongodb = Database::get_mongodb()?;
    mongodb.guilds.send_log(event.guild_id, embed, false).await
}
