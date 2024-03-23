use crate::structs::{api::urban_dictionary::UrbanDictionary, command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match UrbanDictionary::search(ctx.get_string_arg("word")?).await {
        Ok(urban_dictionary) => ctx.respond(urban_dictionary.format(), !ctx.get_bool_arg("show").unwrap_or(false)).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
