use crate::{statics::colors::NOTICE_EMBED_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::GuildEmojisUpdate;

pub async fn handle(event: &GuildEmojisUpdate) -> Result<()> {
    let embed = Embed::new().set_color(NOTICE_EMBED_COLOR).unwrap_or_default().set_title("Server Emojis Updated").set_timestamp(Utc::now());

    Guilds::send_log(event.guild_id, embed).await
}
