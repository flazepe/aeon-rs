use crate::structs::{
    api::dictionary::Dictionary,
    command_context::{CommandContext, CommandInputExt, Input},
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let (word, ephemeral) = match &ctx.input {
        Input::ApplicationCommand { input, res: _ } => (input.get_string_arg("word")?, !input.get_bool_arg("show").unwrap_or(false)),
        Input::MessageCommand { message: _, sender: _, args } => (args.into(), true),
    };

    if word.is_empty() {
        return ctx.respond_error("Please provide a word.", true).await;
    }

    match Dictionary::search(word).await {
        Ok(dictionary) => ctx.respond(dictionary.format(), ephemeral).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
