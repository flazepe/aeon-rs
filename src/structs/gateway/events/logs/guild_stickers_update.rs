use crate::{statics::colors::NOTICE_EMBED_COLOR, structs::database::Database};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::GuildStickersUpdate;

pub async fn handle(event: &GuildStickersUpdate) -> Result<()> {
    let embed =
        Embed::new().set_color(NOTICE_EMBED_COLOR).unwrap_or_default().set_title("Server Stickers Updated").set_timestamp(Utc::now());

    let mongodb = Database::get_mongodb()?;
    mongodb.guilds.send_log(event.guild_id, embed, false).await
}
