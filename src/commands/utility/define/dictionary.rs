use crate::structs::{
    api::dictionary::Dictionary,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use anyhow::{Result, bail};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let (word, ephemeral) = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, _) => (input.get_string_arg("word")?, !input.get_bool_arg("show").unwrap_or(false)),
        AeonCommandInput::MessageCommand(_, args, _) => (args.into(), true),
    };

    if word.is_empty() {
        bail!("Please provide a word.");
    }

    let dictionary = Dictionary::search(word).await?;
    ctx.respond(dictionary.format(), ephemeral).await
}
