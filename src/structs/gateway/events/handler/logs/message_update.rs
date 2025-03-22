use crate::{
    statics::{colors::NOTICE_COLOR, CACHE},
    structs::simple_message::SimpleMessage,
    traits::UserExt,
};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::MessageUpdate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &MessageUpdate) -> Result<(Option<Id<GuildMarker>>, Option<Embed>)> {
    let channels = CACHE.channels.read().unwrap();
    let Some(old_message) =
        channels.get(&event.channel_id.to_string()).and_then(|messages| messages.iter().find(|message| message.id == event.id))
    else {
        return Ok((None, None));
    };

    if old_message.content == event.content {
        return Ok((None, None));
    }

    let embed = Embed::new()
        .set_color(NOTICE_COLOR)
        .unwrap_or_default()
        .set_title("Message Updated")
        .set_description(format!(
            "https://discord.com/channels/{}/{}/{}",
            event.guild_id.map(|guild_id| guild_id.to_string()).as_deref().unwrap_or(""),
            event.channel_id,
            event.id,
        ))
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false)
        .add_field("Old Content", SimpleMessage::from(old_message.clone()).to_string().chars().take(1024).collect::<String>(), false)
        .add_field("New Content", SimpleMessage::from(event.0.clone()).to_string().chars().take(1024).collect::<String>(), false)
        .set_footer(event.author.label(), Some(event.author.display_avatar_url("gif", 4096)));

    Ok((event.guild_id, embed.into()))
}
