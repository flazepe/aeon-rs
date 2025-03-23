use crate::{statics::colors::NOTICE_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::gateway::payload::incoming::GuildStickersUpdate;

pub async fn log(event: &GuildStickersUpdate) -> Result<()> {
    let embed = Embed::new().set_color(NOTICE_COLOR).unwrap_or_default().set_title("Server Stickers Updated");

    Guilds::send_log(event.guild_id, embed).await
}
