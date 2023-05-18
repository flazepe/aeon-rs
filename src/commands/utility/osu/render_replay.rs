use crate::{
    statics::CACHE,
    structs::{
        api::ordr::{statics::ORDR_SKINS, OrdrRender},
        command_context::CommandContext,
    },
    traits::ArgGetters,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.is_autocomplete() {
        return ctx.autocomplete(ORDR_SKINS.iter()).await;
    }

    if CACHE.ordr_rendering_users.read().unwrap().contains_key(&ctx.input.user.id) {
        return ctx.respond_error("You already have an ongoing replay rendering.", true).await;
    }

    ctx.res.defer(false).await?;

    match OrdrRender::new(
        match ctx
            .input
            .get_string_arg("replay-url")
            .or(ctx.input.get_attachment_arg("replay-file").map(|attachment| attachment.url.clone()))
        {
            Ok(url) => url,
            Err(_) => return ctx.respond_error("Please provide an image URL or file.", true).await,
        },
        ctx.input.get_string_arg("skin").ok(),
    )
    .await
    {
        Ok(render) => render.poll_progress(&ctx).await,
        Err(error) => ctx.respond_error(error, false).await,
    }
}
