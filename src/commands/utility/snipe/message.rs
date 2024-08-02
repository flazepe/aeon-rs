use crate::structs::{command_context::CommandContext, snipes::Snipes};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let channel = ctx.get_channel_arg("channel").map_or(ctx.input.channel_id.as_ref().unwrap(), |channel| &channel.id);
    let edit = ctx.get_bool_arg("edit").unwrap_or(false);
    let list = ctx.get_bool_arg("list").unwrap_or(false);
    let permissions = ctx.input.app_permissions;

    match Snipes::new(channel, edit, list, permissions).to_response() {
        Ok(response) => ctx.respond(response, false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
