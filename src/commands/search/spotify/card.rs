use crate::{
    statics::{CACHE, CONFIG},
    structs::{command_context::CommandContext, gateway::cache::SongActivityStyle},
    traits::AvatarUrl,
};
use anyhow::Result;
use serde_json::to_string;
use slashook::structs::utils::File;
use tokio::process::Command;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let mut user = ctx.get_user_arg("member").unwrap_or(&ctx.input.user);

    // Set to author if there's no resolved member
    if ctx.input.resolved.as_ref().and_then(|resolved| resolved.members.as_ref().and_then(|members| members.values().next())).is_none() {
        user = &ctx.input.user;
    }

    let activity;

    {
        let activities = CACHE.spotify.read().unwrap();
        activity = activities.get(&user.id).cloned();
    }

    match activity {
        Some(activity) => {
            ctx.res.defer(false).await?;

            let mut activity = activity.clone();

            // Set to proper style
            if let Ok(style) = ctx.get_string_arg("style").as_deref() {
                activity.style = match style {
                    "classic" => SongActivityStyle::Classic,
                    "nori" => SongActivityStyle::Nori,
                    "rovi" => SongActivityStyle::Rovi,
                    "vxc" => SongActivityStyle::Vxc,
                    // Default card style is nori's
                    _ => SongActivityStyle::Nori,
                };
            }

            // Set to user display avatar if track is local
            if activity.album_cover.is_empty() {
                activity.album_cover = user.display_avatar_url("png", 4096);
            }

            ctx.respond(
                File::new(
                    "image.png",
                    Command::new("node").args([&CONFIG.api.song_card_generator_path, &to_string(&activity)?]).output().await?.stdout,
                ),
                false,
            )
            .await
        },
        None => ctx.respond_error(format!("No Spotify activity found for <@{}>.", user.id), true).await,
    }
}
