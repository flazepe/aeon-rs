use crate::{
    structs::{command_context::CommandContext, unicode::UnicodeCharacter},
    traits::ArgGetters,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match UnicodeCharacter::get(ctx.input.get_string_arg("query")?).await {
        Ok(unicode_character) => ctx.respond(unicode_character.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
