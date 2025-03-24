use crate::structs::{
    api::steam::Steam,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    match Steam::get_user(input.get_string_arg("user")?).await {
        Ok(user) => ctx.respond(user.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
