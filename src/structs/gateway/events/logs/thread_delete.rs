use crate::{statics::colors::ERROR_COLOR, structs::database::guilds::Guilds};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::gateway::payload::incoming::ThreadDelete;

pub async fn handle(event: &ThreadDelete) -> Result<()> {
    let embed = Embed::new()
        .set_color(ERROR_COLOR)
        .unwrap_or_default()
        .set_title("Thread Deleted")
        .set_description(format!("<#{thread_id}> ({thread_id})", thread_id = event.id))
        .add_field("Type", format!("{:?}", event.kind), false)
        .add_field("Parent", format!("<#{parent_id}> ({parent_id})", parent_id = event.parent_id), false);

    Guilds::send_log(event.guild_id, embed).await
}
