use crate::{functions::eien, structs::command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    ctx.res.defer(false).await?;
    ctx.respond(eien(&ctx.get_string_arg("command")?, &[&ctx.get_string_arg("args")?]).await?, false).await
}
