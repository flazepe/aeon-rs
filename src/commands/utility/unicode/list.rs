use crate::{
    structs::{command_context::CommandContext, unicode::UnicodeCharacters},
    traits::ArgGetters,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    ctx.respond(UnicodeCharacters::get(ctx.input.get_string_arg("text")?).format(), false).await
}
