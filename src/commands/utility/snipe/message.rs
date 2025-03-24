use crate::structs::{
    command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
    snipes::Snipes,
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };
    let channel = input.get_channel_arg("channel").map_or(input.channel_id.as_ref().unwrap(), |channel| &channel.id);
    let edit = input.get_bool_arg("edit").unwrap_or(false);
    let list = input.get_bool_arg("list").unwrap_or(false);
    let permissions = input.app_permissions;

    match Snipes::new(channel, edit, list, permissions).to_response() {
        Ok(response) => ctx.respond(response, false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
