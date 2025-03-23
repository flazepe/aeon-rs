use crate::{statics::colors::SUCCESS_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::gateway::payload::incoming::ChannelCreate;

pub async fn handle(event: &ChannelCreate) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };

    let embed = Embed::new()
        .set_color(SUCCESS_COLOR)
        .unwrap_or_default()
        .set_title("Channel Created")
        .set_description(format!("<#{channel_id}> ({channel_id})", channel_id = event.id))
        .add_field("Name", format!("#{}", event.name.as_deref().unwrap_or("unknown")), false);

    Guilds::send_log(guild_id, embed).await
}
