use crate::{
    functions::{hastebin, limit_strings},
    structs::{command_context::CommandContext, unicode::Unicode},
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let mut formatted = Unicode::list(ctx.get_string_arg("text")?).format();

    if formatted.len() > 2000 {
        let extra = format!("\n\nFull list: {}", hastebin(&formatted).await?);
        formatted = limit_strings(formatted.split('\n'), '\n', 2000 - extra.len()) + &extra;
    }

    ctx.respond(formatted, false).await
}
