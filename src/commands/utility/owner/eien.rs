use crate::{
    functions::eien,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(..) = &ctx.command_input else { return Ok(()) };
    ctx.defer(false).await?;
    ctx.respond(eien(ctx.get_string_arg("command")?, &[&ctx.get_string_arg("args")?]).await?, false).await
}
