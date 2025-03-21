use crate::structs::{command_context::CommandContext, database::guilds::Guilds};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let guild = Guilds::get(ctx.input.guild_id.as_ref().unwrap()).await?;
    ctx.respond(format!("```rs\n{guild:#?}```"), true).await
}
