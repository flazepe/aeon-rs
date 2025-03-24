use crate::structs::{
    api::saucenao::SauceNaoSearch,
    command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };
    let Ok(url) = input.get_string_arg("image-url").or(input.get_attachment_arg("image-file").map(|attachment| attachment.url.clone()))
    else {
        return ctx.respond_error("Please provide an image URL or file.", true).await;
    };

    match SauceNaoSearch::query(url).await {
        Ok(saucenao_search) => ctx.respond(saucenao_search.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
