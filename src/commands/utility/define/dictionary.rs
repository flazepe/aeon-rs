use crate::structs::{api::dictionary::Dictionary, command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Dictionary::search(ctx.get_string_arg("word")?).await {
        Ok(dictionary) => ctx.respond(dictionary.format(), !ctx.get_bool_arg("show").unwrap_or(false)).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
