use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput},
    database::guilds::Guilds,
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };
    let guild = Guilds::get(input.guild_id.as_ref().unwrap()).await?;
    ctx.respond(format!("```rs\n{guild:#?}```"), true).await
}
