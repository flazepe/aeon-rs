use crate::structs::{command_context::CommandContext, snipes::ReactionSnipes};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let message = ctx.get_string_arg("message")?;

    match ReactionSnipes::new(ctx.input.guild_id.as_ref().unwrap(), message.split("/").last().unwrap()).to_response() {
        Ok(response) => ctx.respond(response, false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
