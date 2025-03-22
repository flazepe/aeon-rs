use crate::{functions::label_num, statics::colors::ERROR_COLOR};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::MessageDeleteBulk,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &MessageDeleteBulk) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let embed = Embed::new()
        .set_color(ERROR_COLOR)
        .unwrap_or_default()
        .set_title(format!("{} {} Deleted", event.ids.len(), label_num(event.ids.len(), "Message", "Messages")))
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false);

    Ok((event.guild_id, embed))
}
