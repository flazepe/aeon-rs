use crate::{
    functions::eien,
    structs::command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };
    ctx.defer(false).await?;
    ctx.respond(eien(input.get_string_arg("command")?, &[&input.get_string_arg("args")?]).await?, false).await
}
