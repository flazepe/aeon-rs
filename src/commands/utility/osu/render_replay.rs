use crate::{
    statics::CACHE,
    structs::{
        api::ordr::{OrdrRender, statics::ORDR_SKINS},
        command_context::{CommandContext, CommandInputExt, Input},
    },
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand(input,  _) = &ctx.input else { return Ok(()) };

    if input.is_autocomplete() {
        return ctx.autocomplete(ORDR_SKINS.iter()).await;
    }

    if CACHE.ordr_rendering_users.read().unwrap().contains_key(&input.user.id) {
        return ctx.respond_error("You already have an ongoing replay rendering.", true).await;
    }

    ctx.defer(false).await?;

    let Ok(replay_url) =
        input.get_string_arg("replay-url").or(input.get_attachment_arg("replay-file").map(|attachment| attachment.url.clone()))
    else {
        return ctx.respond_error("Please provide an image URL or file.", true).await;
    };

    let skin = input.get_string_arg("skin").ok();

    match OrdrRender::new(replay_url, skin).await {
        Ok(render) => render.poll_progress(&ctx).await,
        Err(error) => ctx.respond_error(error, false).await,
    }
}
