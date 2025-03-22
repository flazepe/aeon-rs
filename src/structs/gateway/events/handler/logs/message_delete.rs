use crate::{
    statics::{colors::ERROR_COLOR, CACHE},
    structs::simple_message::SimpleMessage,
    traits::UserExt,
};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::MessageDelete,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &MessageDelete) -> Result<(Option<Id<GuildMarker>>, Option<Embed>)> {
    let mut embed = Embed::new().set_color(ERROR_COLOR).unwrap_or_default().set_title("Message Deleted").add_field(
        "Channel",
        format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id),
        false,
    );

    let channels = CACHE.channels.read().unwrap();
    let old_message =
        channels.get(&event.channel_id.to_string()).and_then(|messages| messages.iter().find(|message| message.id == event.id));

    if let Some(old_message) = old_message {
        embed = embed
            .add_field("Content", SimpleMessage::from(old_message.clone()), false)
            .set_footer(old_message.author.label(), Some(old_message.author.display_avatar_url("gif", 4096)));
    }

    Ok((event.guild_id, embed.into()))
}
