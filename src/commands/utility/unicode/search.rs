use crate::{
    functions::{hastebin, limit_strings},
    structs::{
        command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
        unicode::Unicode,
    },
};
use anyhow::{Result, bail};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let character = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, _) => input.get_string_arg("text")?,
        AeonCommandInput::MessageCommand(_, args, _) => args.into(),
    };

    if character.is_empty() {
        bail!("Please provide a character.");
    }

    ctx.defer(false).await?;

    let mut formatted = Unicode::search(character).await?.format();

    if formatted.len() > 2000 {
        let extra = format!("\n\nFull list: {}", hastebin(&formatted).await?);
        formatted = limit_strings(formatted.split('\n'), '\n', 2000 - extra.len()) + &extra;
    }

    ctx.respond(formatted, false).await
}
