use crate::{statics::colors::NOTICE_EMBED_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::ChannelPinsUpdate;

pub async fn handle(event: &ChannelPinsUpdate) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };

    let embed = Embed::new()
        .set_color(NOTICE_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("Channel Pins Updated")
        .set_description(format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id))
        .set_timestamp(Utc::now());

    Guilds::send_log(guild_id, embed, false).await
}
