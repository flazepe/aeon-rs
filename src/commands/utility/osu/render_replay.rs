use crate::{
    statics::CACHE,
    structs::{
        api::ordr::{OrdrRender, statics::ORDR_SKINS},
        command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    },
};
use anyhow::{Result, bail};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    if input.is_autocomplete() {
        return ctx.autocomplete(ORDR_SKINS.iter()).await;
    }

    if CACHE.ordr_rendering_users.read().unwrap().contains_key(&input.user.id) {
        bail!("You already have an ongoing replay rendering.");
    }

    ctx.defer(false).await?;

    let Ok(replay_url) =
        input.get_string_arg("replay-url").or(input.get_attachment_arg("replay-file").map(|attachment| attachment.url.clone()))
    else {
        bail!("Please provide an image URL or file.");
    };

    let skin = input.get_string_arg("skin").ok();
    let render = OrdrRender::new(replay_url, skin).await?;

    render.poll_progress(ctx).await
}
