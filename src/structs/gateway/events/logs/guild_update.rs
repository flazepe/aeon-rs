use crate::statics::{MONGODB, colors::NOTICE_EMBED_COLOR};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::GuildUpdate;

pub async fn handle(event: &GuildUpdate) -> Result<()> {
    let embed = Embed::new().set_color(NOTICE_EMBED_COLOR).unwrap_or_default().set_title("Server Updated").set_timestamp(Utc::now());

    let mongodb = MONGODB.get().unwrap();
    mongodb.guilds.send_log(event.id, embed, false).await
}
