use crate::structs::{command_context::CommandContext, snipes::Snipes};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Snipes::new(
        ctx.get_channel_arg("channel").map_or_else(|_| ctx.input.channel_id.as_ref().unwrap(), |channel| &channel.id),
        ctx.get_bool_arg("edit").unwrap_or(false),
        ctx.get_bool_arg("list").unwrap_or(false),
    )
    .to_response()
    {
        Ok(response) => ctx.respond(response, false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
