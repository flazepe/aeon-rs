use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput},
    snipes::ReactionSnipes,
};
use anyhow::Result;
use slashook::structs::Permissions;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let Some(guild_id) = ctx.get_guild_id() else { return Ok(()) };
    let channel_id = ctx.get_channel_id();

    let message = ctx.get_string_arg("message")?;
    let mut split = message.split('/').rev();
    let (message_id, channel_id) = (split.next().unwrap(), split.next().unwrap_or(&channel_id));
    let permissions = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, _) => input.app_permissions,
        AeonCommandInput::MessageCommand(_, _, _) => Permissions::empty(),
    };

    let response = ReactionSnipes::new(guild_id, channel_id, message_id, permissions).to_response()?;
    ctx.respond(response, false).await
}
