use crate::{
    functions::format_timestamp,
    macros::yes_no,
    statics::colors::{ERROR_EMBED_COLOR, NOTICE_EMBED_COLOR},
    structs::database::guilds::Guilds,
    traits::{UserAvatarExt, UserExt},
};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use twilight_model::gateway::payload::incoming::VoiceStateUpdate;

pub async fn handle(event: &VoiceStateUpdate) -> Result<()> {
    let Some(guild_id) = event.guild_id else { return Ok(()) };

    let mut embed = Embed::new()
        .add_field("Muted", format!("Self? {}\nServer? {}", yes_no!(event.self_mute), yes_no!(event.mute)), true)
        .add_field("Deafened", format!("Self? {}\nServer? {}", yes_no!(event.self_deaf), yes_no!(event.deaf)), true)
        .add_field("Suppressed (for stage channels)", yes_no!(event.suppress), false);

    if let Some(request_to_speak_timestamp) = event.request_to_speak_timestamp {
        embed = embed.add_field("Requested to speak", format_timestamp(request_to_speak_timestamp.as_secs(), true), false);
    }

    embed = embed.add_field("Streaming", yes_no!(event.self_stream), true).add_field("Camera On", yes_no!(event.self_video), true);

    if let Some(channel_id) = event.channel_id {
        embed = embed
            .set_color(NOTICE_EMBED_COLOR)
            .unwrap_or_default()
            .set_title("Voice State Updated")
            .set_description(format!("<#{channel_id}> ({channel_id})"));
    } else {
        embed = embed.set_color(ERROR_EMBED_COLOR).unwrap_or_default().set_title("Left Voice Channel");
    }

    if let Some(member) = &event.member {
        embed = embed.set_footer(member.user.label(), Some(member.user.display_avatar_url("gif", 4096)));
    } else {
        embed = embed.set_footer(format!("User ID: {}", event.user_id), None::<String>);
    }

    embed = embed.set_timestamp(Utc::now());

    Guilds::send_log(guild_id, embed).await
}
