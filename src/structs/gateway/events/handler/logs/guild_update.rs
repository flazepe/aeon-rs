use crate::{statics::colors::NOTICE_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::gateway::payload::incoming::GuildUpdate;

pub async fn log(event: &GuildUpdate) -> Result<()> {
    let embed = Embed::new().set_color(NOTICE_COLOR).unwrap_or_default().set_title("Server Updated");

    Guilds::send_log(event.id, embed).await
}
