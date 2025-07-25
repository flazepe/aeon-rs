use crate::{statics::colors::ERROR_EMBED_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::InviteDelete;

pub async fn handle(event: &InviteDelete) -> Result<()> {
    let embed = Embed::new()
        .set_color(ERROR_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("Invite Deleted")
        .set_description(format!("https://discord.gg/{}", event.code))
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false)
        .set_timestamp(Utc::now());

    Guilds::send_log(event.guild_id, embed, false).await
}
