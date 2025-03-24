use crate::structs::{
    api::dictionary::Dictionary,
    command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let (word, ephemeral) = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input,  _) => (input.get_string_arg("word")?, !input.get_bool_arg("show").unwrap_or(false)),
        AeonCommandInput::MessageCommand(_, args, _)   => (args.into(), true),
    };

    if word.is_empty() {
        return ctx.respond_error("Please provide a word.", true).await;
    }

    match Dictionary::search(word).await {
        Ok(dictionary) => ctx.respond(dictionary.format(), ephemeral).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
