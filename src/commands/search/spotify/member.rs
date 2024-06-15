use crate::{functions::eien, statics::CACHE, structs::command_context::CommandContext, traits::UserExt};
use anyhow::Result;
use serde_json::to_string;

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
            if let Ok(style) = ctx.get_string_arg("card").as_deref() {
                activity.style = style.into();
            }

            // Set to user display avatar if track has empty album cover
            if activity.album_cover.is_empty() {
                activity.album_cover = user.display_avatar_url("png", 4096);
            }

            // Collapse if requested
            if ctx.get_bool_arg("collapse").unwrap_or(false) {
                activity.timestamps = None;
            }

            ctx.respond(eien("song-card", &[&to_string(&activity)?]).await?, false).await
        },
        None => ctx.respond_error(format!("No Spotify activity found for <@{}>.", user.id), true).await,
    }
}
