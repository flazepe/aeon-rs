use crate::{
    functions::eien,
    structs::command_context::{CommandContext, CommandInputExt, Input},
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };
    ctx.defer(false).await?;
    ctx.respond(eien(input.get_string_arg("command")?, &[&input.get_string_arg("args")?]).await?, false).await
}
