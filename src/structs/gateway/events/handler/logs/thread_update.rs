use crate::statics::colors::NOTICE_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::ThreadUpdate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &ThreadUpdate) -> Result<(Option<Id<GuildMarker>>, Option<Embed>)> {
    let mut embed = Embed::new()
        .set_color(NOTICE_COLOR)
        .unwrap_or_default()
        .set_title("Thread Updated")
        .set_description(format!("<#{thread_id}> ({thread_id})", thread_id = event.id))
        .add_field("Name", event.name.as_deref().unwrap_or("unknown"), false)
        .add_field("Type", format!("{:?}", event.kind), false);

    if let Some(parent_id) = event.parent_id {
        embed = embed.add_field("Parent", format!("<#{parent_id}> ({parent_id})"), false);
    }

    Ok((event.guild_id, embed.into()))
}
