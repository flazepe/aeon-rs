use crate::structs::{command_context::CommandContext, unicode::UnicodeCharacters};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    ctx.respond(UnicodeCharacters::get(ctx.get_string_arg("text")?).format(), false).await
}
