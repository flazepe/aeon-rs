use crate::{
    statics::{CACHE, colors::ERROR_EMBED_COLOR},
    structs::{database::guilds::Guilds, simple_message::SimpleMessage},
    traits::UserExt,
};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::MessageDelete;

pub async fn handle(event: &MessageDelete) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };
    let mut embed = Embed::new().set_color(ERROR_EMBED_COLOR).unwrap_or_default().set_title("Message Deleted").add_field(
        "Channel",
        format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id),
        false,
    );

    let old_message = {
        let channels = CACHE.channels.read().unwrap();
        channels.get(&event.channel_id.to_string()).and_then(|messages| messages.iter().find(|message| message.id == event.id)).cloned()
    };

    if let Some(old_message) = old_message {
        embed = embed
            .add_field("Content", SimpleMessage::from(old_message.clone()), false)
            .set_footer(old_message.author.label(), Some(old_message.author.display_avatar_url("gif", 4096)));
    }

    embed = embed.set_timestamp(Utc::now());

    Guilds::send_log(guild_id, embed).await
}
