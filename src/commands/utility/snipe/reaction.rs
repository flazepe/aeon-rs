use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    snipes::ReactionSnipes,
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let guild_id = input.guild_id.as_ref().unwrap();

    let message = input.get_string_arg("message")?;
    let mut split = message.split('/').rev();
    let (message_id, channel_id) = (split.next().unwrap(), split.next().unwrap_or(input.channel_id.as_ref().unwrap()));

    let permissions = input.app_permissions;
    let response = ReactionSnipes::new(guild_id, channel_id, message_id, permissions).to_response()?;

    ctx.respond(response, false).await
}
