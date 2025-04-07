use crate::{
    statics::CACHE,
    structs::{
        api::ordr::{OrdrRender, statics::ORDR_SKINS},
        command_context::{AeonCommandContext, AeonCommandInput},
    },
};
use anyhow::{Context, Result, bail};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let mut replay_url = ctx.get_string_arg("replay-url", 0, true);
    let mut skin = None::<String>;

    if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input {
        if input.is_autocomplete() {
            return ctx.autocomplete(ORDR_SKINS.iter()).await;
        }

        replay_url = replay_url.or(ctx.get_attachment_arg("replay-file").map(|attachment| attachment.url.clone()));
        skin = ctx.get_string_arg("skin", 0, true).ok();
    }

    if CACHE.ordr_rendering_users.read().unwrap().contains_key(&ctx.get_user_id()) {
        bail!("You already have an ongoing replay rendering.");
    }

    ctx.defer(false).await?;

    // let render = OrdrRender::new(replay_url.context("Please provide an image URL or file.")?, skin).await?;
    // render.poll_progress(ctx).await

    OrdrRender::new(replay_url.context("Please provide an image URL or file.")?, skin).await?;
    ctx.respond_success("Render request sent! Check out <https://ordr.issou.best/renders>.", false).await
}
