use crate::structs::{
    api::saucenao::SauceNaoSearch,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let mut image_url = ctx.get_string_arg("image-url");

    if let AeonCommandInput::ApplicationCommand(_, _) = &ctx.command_input {
        image_url = image_url.or(ctx.get_attachment_arg("image-file").map(|attachment| attachment.url.clone()));
    }

    let saucenao_search = SauceNaoSearch::query(image_url?).await?;
    ctx.respond(saucenao_search.format(), false).await
}
