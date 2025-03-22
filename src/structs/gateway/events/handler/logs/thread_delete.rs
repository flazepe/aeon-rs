use crate::statics::colors::ERROR_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::ThreadDelete,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &ThreadDelete) -> Result<(Option<Id<GuildMarker>>, Option<Embed>)> {
    let embed = Embed::new()
        .set_color(ERROR_COLOR)
        .unwrap_or_default()
        .set_title("Thread Deleted")
        .set_description(format!("<#{thread_id}> ({thread_id})", thread_id = event.id))
        .add_field("Type", format!("{:?}", event.kind), false)
        .add_field("Parent", format!("<#{parent_id}> ({parent_id})", parent_id = event.parent_id), false);

    Ok((event.guild_id.into(), embed.into()))
}
