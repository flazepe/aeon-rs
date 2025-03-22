use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    functions::{format_timestamp, TimestampFormat},
    statics::colors::SUCCESS_COLOR,
    traits::UserExt,
};
use anyhow::Result;
use slashook::structs::embeds::Embed;
use twilight_model::{
    gateway::payload::incoming::InviteCreate,
    id::{marker::GuildMarker, Id},
};

pub async fn log(event: &InviteCreate) -> Result<(Option<Id<GuildMarker>>, Option<Embed>)> {
    let mut embed = Embed::new()
        .set_color(SUCCESS_COLOR)
        .unwrap_or_default()
        .set_title("Invite Created")
        .set_description(format!("https://discord.gg/{}", event.code))
        .add_field("Channel", format!("<#{channel_id}> ({channel_id})", channel_id = event.channel_id), false)
        .add_field("Max Uses", if event.max_uses == 0 { "Unlimited".into() } else { event.max_uses.to_string() }, false)
        .add_field(
            "Valid Until",
            if event.max_age == 0 {
                "Forever".into()
            } else {
                format_timestamp(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + event.max_age, TimestampFormat::Full)
            },
            false,
        );

    if let Some(target_user) = &event.target_user {
        embed = embed.add_field("Target User", format!("<@{user_id}> ({user_id})", user_id = target_user.id), false);
    }

    if let Some(target_user_type) = &event.target_user_type {
        embed = embed.add_field("Target User Type", format!("{:?}", target_user_type), false);
    }

    if let Some(inviter) = &event.inviter {
        embed = embed.set_footer(inviter.label(), Some(inviter.display_avatar_url("gif", "4096")));
    }

    Ok((event.guild_id.into(), embed.into()))
}
