use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput},
    snipes::Snipes,
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let channel = ctx.get_channel_arg("channel").map_or(input.channel_id.as_ref().unwrap(), |channel| &channel.id);
    let edit = ctx.get_bool_arg("edit").unwrap_or(false);
    let list = ctx.get_bool_arg("list").unwrap_or(false);
    let permissions = input.app_permissions;
    let response = Snipes::new(channel, edit, list, permissions).to_response()?;

    ctx.respond(response, false).await
}
