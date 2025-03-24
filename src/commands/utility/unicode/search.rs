use crate::{
    functions::{hastebin, limit_strings},
    structs::{
        command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
        unicode::Unicode,
    },
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let character = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input,  _) => input.get_string_arg("text")?,
        AeonCommandInput::MessageCommand(_, args, _)   => args.into(),
    };

    if character.is_empty() {
        return ctx.respond_error("Please provide a character.", true).await;
    }

    ctx.defer(false).await?;

    let mut formatted = match Unicode::search(character).await {
        Ok(unicode) => unicode.format(),
        Err(error) => return ctx.respond_error(error, true).await,
    };

    if formatted.len() > 2000 {
        let extra = format!("\n\nFull list: {}", hastebin(&formatted).await?);
        formatted = limit_strings(formatted.split('\n'), '\n', 2000 - extra.len()) + &extra;
    }

    ctx.respond(formatted, false).await
}
