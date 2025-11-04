use crate::{statics::colors::ERROR_EMBED_COLOR, structs::database::Database, traits::UserExt};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::MemberRemove;

pub async fn handle(event: &MemberRemove) -> Result<()> {
    let embed = Embed::new()
        .set_color(ERROR_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("Member Left")
        .set_description(format!("<@{}>", event.user.id))
        .add_field("Username", event.user.label(), false)
        .set_timestamp(Utc::now());

    let mongodb = Database::get_mongodb()?;
    mongodb.guilds.send_log(event.guild_id, embed, false).await
}
