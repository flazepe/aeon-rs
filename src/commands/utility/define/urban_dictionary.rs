use crate::structs::{
    api::urban_dictionary::UrbanDictionary,
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

    match UrbanDictionary::search(word).await {
        Ok(urban_dictionary) => ctx.respond(urban_dictionary.format(), ephemeral).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
