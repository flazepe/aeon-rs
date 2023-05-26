use crate::structs::{command_context::CommandContext, database::tags::Tags};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Tags::delete(ctx.get_string_arg("tag")?, ctx.input.guild_id.as_ref().unwrap(), ctx.input.member.as_ref().unwrap()).await {
        Ok(response) => ctx.respond_success(response, true).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
