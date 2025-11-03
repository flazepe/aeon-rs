use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput},
    snipes::ReactionSnipes,
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let Some(guild_id) = ctx.get_guild_id() else { return Ok(()) };
    let channel_id = ctx.get_channel_arg("channel").map_or(input.channel_id.as_ref().unwrap(), |channel| &channel.id);
    let permissions = input.app_permissions;
    let response = ReactionSnipes::new(guild_id, channel_id, permissions).await.to_response().await?;

    ctx.respond(response, false).await
}
