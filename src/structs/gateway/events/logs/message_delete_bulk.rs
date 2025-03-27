use crate::{functions::label_num, statics::colors::ERROR_EMBED_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

pub async fn handle(event: &MessageDeleteBulk) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };

    let embed = Embed::new()
        .set_color(ERROR_EMBED_COLOR)
        .unwrap_or_default()
        .set_title(format!("{} Deleted", label_num(event.ids.len(), "Message", "Messages")))
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false)
        .set_timestamp(Utc::now());

    Guilds::send_log(guild_id, embed).await
}
