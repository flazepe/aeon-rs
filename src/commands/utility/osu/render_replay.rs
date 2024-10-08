use crate::{
    statics::CACHE,
    structs::{
        api::ordr::{statics::ORDR_SKINS, OrdrRender},
        command_context::CommandContext,
    },
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.is_autocomplete() {
        return ctx.autocomplete(ORDR_SKINS.iter()).await;
    }

    if CACHE.ordr_rendering_users.read().unwrap().contains_key(&ctx.input.user.id) {
        return ctx.respond_error("You already have an ongoing replay rendering.", true).await;
    }

    ctx.defer(false).await?;

    let Ok(replay_url) =
        ctx.get_string_arg("replay-url").or(ctx.get_attachment_arg("replay-file").map(|attachment| attachment.url.clone()))
    else {
        return ctx.respond_error("Please provide an image URL or file.", true).await;
    };

    let skin = ctx.get_string_arg("skin").ok();

    match OrdrRender::new(replay_url, skin).await {
        Ok(render) => render.poll_progress(&ctx).await,
        Err(error) => ctx.respond_error(error, false).await,
    }
}
