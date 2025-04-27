use crate::{
    functions::format_timestamp,
    statics::colors::SUCCESS_EMBED_COLOR,
    structs::database::guilds::Guilds,
    traits::{UserAvatarExt, UserExt},
};
use anyhow::Result;
use slashook::{chrono::Utc, structs::embeds::Embed};
use std::time::{SystemTime, UNIX_EPOCH};
use twilight_model::gateway::payload::incoming::InviteCreate;

pub async fn handle(event: &InviteCreate) -> Result<()> {
    let mut embed = Embed::new()
        .set_color(SUCCESS_EMBED_COLOR)
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
                format_timestamp(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + event.max_age, true)
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

    embed = embed.set_timestamp(Utc::now());

    Guilds::send_log(event.guild_id, embed).await
}
