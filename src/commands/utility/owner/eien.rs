use crate::{
    functions::eien,
    structs::command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    ctx.defer(false).await?;
    ctx.respond(eien(input.get_string_arg("command")?, &[&input.get_string_arg("args")?]).await?, false).await
}
