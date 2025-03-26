use crate::structs::{
    api::saucenao::SauceNaoSearch,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use anyhow::{Result, bail};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let url = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, _) => input
            .get_string_arg("image-url")
            .or(input.get_attachment_arg("image-file").map(|attachment| attachment.url.clone()))
            .unwrap_or_default(),
        AeonCommandInput::MessageCommand(_, args, _) => args.into(),
    };

    if url.is_empty() {
        bail!("Please provide an image URL or file.");
    }

    let saucenao_search = SauceNaoSearch::query(url).await?;

    ctx.respond(saucenao_search.format(), false).await
}
