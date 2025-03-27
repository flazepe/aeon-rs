use crate::{statics::colors::SUCCESS_EMBED_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::ThreadCreate;

pub async fn handle(event: &ThreadCreate) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };

    let mut embed = Embed::new()
        .set_color(SUCCESS_EMBED_COLOR)
        .unwrap_or_default()
        .set_title("Thread Created")
        .set_description(format!("<#{thread_id}> ({thread_id})", thread_id = event.id))
        .add_field("Name", event.name.as_deref().unwrap_or("unknown"), false)
        .add_field("Type", format!("{:?}", event.kind), false);

    if let Some(parent_id) = event.parent_id {
        embed = embed.add_field("Parent", format!("<#{parent_id}> ({parent_id})"), false);
    }

    embed = embed.set_timestamp(Utc::now());

    Guilds::send_log(guild_id, embed).await
}
