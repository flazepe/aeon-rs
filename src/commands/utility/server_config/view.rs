use crate::{
    statics::MONGODB,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let mongodb = MONGODB.get().unwrap();
    let guild = mongodb.guilds.get(input.guild_id.as_ref().unwrap()).await?;
    ctx.respond(format!("```rs\n{guild:#?}```"), true).await
}
