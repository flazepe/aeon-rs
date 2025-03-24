use crate::structs::{
    command_context::{CommandContext, Input},
    database::guilds::Guilds,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };
    let guild = Guilds::get(input.guild_id.as_ref().unwrap()).await?;
    ctx.respond(format!("```rs\n{guild:#?}```"), true).await
}
