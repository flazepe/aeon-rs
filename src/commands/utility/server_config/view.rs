use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput},
    database::guilds::Guilds,
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };
    let guild = Guilds::get(input.guild_id.as_ref().unwrap()).await?;
    ctx.respond(format!("```rs\n{guild:#?}```"), true).await
}
