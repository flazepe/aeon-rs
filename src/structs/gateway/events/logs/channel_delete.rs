use crate::statics::{MONGODB, colors::ERROR_EMBED_COLOR};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::ChannelDelete;

pub async fn handle(event: &ChannelDelete) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };

    let embed = Embed::new()
        .set_color(ERROR_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("Channel Deleted")
        .set_description(format!("<#{channel_id}> ({channel_id})", channel_id = event.id))
        .add_field("Name", format!("#{}", event.name.as_deref().unwrap_or("unknown")), false)
        .set_timestamp(Utc::now());

    let mongodb = MONGODB.get().unwrap();
    mongodb.guilds.send_log(guild_id, embed, false).await
}
