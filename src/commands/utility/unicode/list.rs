use crate::{
    functions::{hastebin, limit_strings},
    structs::{
        command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
        unicode::Unicode,
    },
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let text = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input,  _) => input.get_string_arg("text")?,
        AeonCommandInput::MessageCommand(_, args, _)   => args.into(),
    };

    if text.is_empty() {
        return ctx.respond_error("Please provide a text.", true).await;
    }

    let mut formatted = Unicode::list(text).format();

    if formatted.len() > 2000 {
        let extra = format!("\n\nFull list: {}", hastebin(&formatted).await?);
        formatted = limit_strings(formatted.split('\n'), '\n', 2000 - extra.len()) + &extra;
    }

    ctx.respond(formatted, false).await
}
