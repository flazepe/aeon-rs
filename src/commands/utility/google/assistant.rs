use crate::structs::{api::google::Google, command_context::CommandContext};
use anyhow::Result;
use slashook::structs::utils::File;

pub async fn run(ctx: CommandContext) -> Result<()> {
    ctx.res.defer(false).await?;

    match Google::query_assistant(ctx.get_string_arg("query")?).await {
        Ok(image) => ctx.respond(File::new("image.png", image), false).await,
        Err(error) => ctx.respond_error(error, false).await,
    }
}
