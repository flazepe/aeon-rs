use crate::structs::{api::steam::Steam, command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Steam::get_user(ctx.get_string_arg("user")?).await {
        Ok(user) => ctx.respond(user.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
