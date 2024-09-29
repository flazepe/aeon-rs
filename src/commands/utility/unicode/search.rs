use crate::{
    functions::{hastebin, limit_strings},
    structs::{command_context::CommandContext, unicode::Unicode},
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    ctx.defer(false).await?;

    let mut formatted = match Unicode::search(ctx.get_string_arg("character")?).await {
        Ok(unicode) => unicode.format(),
        Err(error) => return ctx.respond_error(error, true).await,
    };

    if formatted.len() > 2000 {
        let extra = format!("\n\nFull list: {}", hastebin(&formatted).await?);
        formatted = limit_strings(formatted.split('\n'), '\n', 2000 - extra.len()) + &extra;
    }

    ctx.respond(formatted, false).await
}
