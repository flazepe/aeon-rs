use crate::statics::colors::SUCCESS_COLOR;
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::ThreadCreate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &ThreadCreate) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let mut embed = Embed::new()
        .set_color(SUCCESS_COLOR)
        .unwrap_or_default()
        .set_title("Thread Created")
        .set_description(format!("<#{thread_id}> ({thread_id})", thread_id = event.id))
        .add_field("Name", event.name.as_deref().unwrap_or("unknown"), false)
        .add_field("Type", format!("{:?}", event.kind), false);

    if let Some(parent_id) = event.parent_id {
        embed = embed.add_field("Parent", format!("<#{parent_id}> ({parent_id})"), false);
    }

    Ok((event.guild_id, embed))
}
