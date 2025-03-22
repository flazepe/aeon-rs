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

pub async fn log(event: &MessageUpdate) -> Result<(Option<Id<GuildMarker>>, Embed)> {
    let mut embed = Embed::new()
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
        .set_footer(event.author.label(), Some(event.author.display_avatar_url("gif", 4096)));

    let channels = CACHE.channels.read().unwrap();
    let old_message =
        channels.get(&event.channel_id.to_string()).and_then(|messages| messages.iter().find(|message| message.id == event.id));

    if let Some(old_message) = old_message {
        embed = embed.add_field(
            "Old Content",
            SimpleMessage::from(old_message.clone()).to_string().chars().take(1024).collect::<String>(),
            false,
        );
    }

    embed = embed.add_field("New Content", SimpleMessage::from(event.0.clone()).to_string().chars().take(1024).collect::<String>(), false);

    Ok((event.guild_id, embed))
}
