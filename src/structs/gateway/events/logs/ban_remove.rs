use crate::{statics::colors::SUCCESS_EMBED_COLOR, structs::database::Database, traits::UserExt};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::BanRemove;

pub async fn handle(event: &BanRemove) -> Result<()> {
    let embed = Embed::new()
        .set_color(SUCCESS_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("User Unbanned")
        .set_description(format!("<@{}>", event.user.id))
        .add_field("Username", event.user.label(), false)
        .set_timestamp(Utc::now());

    let mongodb = Database::get_mongodb()?;
    mongodb.guilds.send_log(event.guild_id, embed, false).await
}
