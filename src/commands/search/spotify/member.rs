use crate::{
    functions::eien,
    statics::CACHE,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
    traits::UserExt,
};
use anyhow::{Result, bail};
use serde_json::to_string;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let mut user = ctx.get_user_arg("member").unwrap_or(&input.user);

    // Set to author if there's no resolved member
    if input.resolved.as_ref().and_then(|resolved| resolved.members.as_ref().and_then(|members| members.values().next())).is_none() {
        user = &input.user;
    }

    let Some(mut activity) = CACHE.song_activities.read().unwrap().get(&user.id).cloned() else {
        bail!(format!("No Spotify activity found for {}.", user.mention()));
    };

    ctx.defer(false).await?;

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
}
