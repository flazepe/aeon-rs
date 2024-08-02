use crate::structs::{command_context::CommandContext, database::tags::Tags};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let name = ctx.get_string_arg("tag")?;
    let guild_id = ctx.input.guild_id.as_ref().unwrap();
    let modifier = ctx.input.member.as_ref().unwrap();

    match Tags::delete(name, guild_id, modifier).await {
        Ok(response) => ctx.respond_success(response, true).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
