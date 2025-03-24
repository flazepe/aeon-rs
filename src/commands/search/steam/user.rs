use crate::structs::{
    api::steam::Steam,
    command_context::{CommandContext, CommandInputExt, Input},
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand(input,  _) = &ctx.input else { return Ok(()) };

    match Steam::get_user(input.get_string_arg("user")?).await {
        Ok(user) => ctx.respond(user.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
