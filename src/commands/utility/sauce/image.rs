use crate::{structs::api::saucenao::SauceNaoSearch, structs::command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Ok(url) = ctx.get_string_arg("image-url").or(ctx.get_attachment_arg("image-file").map(|attachment| attachment.url.clone())) else {
        return ctx.respond_error("Please provide an image URL or file.", true).await;
    };

    match SauceNaoSearch::query(url).await {
        Ok(saucenao_search) => ctx.respond(saucenao_search.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
