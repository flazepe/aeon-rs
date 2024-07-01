use crate::structs::{command_context::CommandContext, unicode::Unicode};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Unicode::search(ctx.get_string_arg("character")?).await {
        Ok(unicode) => ctx.respond(unicode.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
