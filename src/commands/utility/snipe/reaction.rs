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
    let send_list = ctx.get_bool_arg("list").unwrap_or(false);
    let permissions = input.app_permissions;
    let response = ReactionSnipes::new(guild_id, channel_id, send_list, permissions).await.to_response().await?;

    ctx.respond(response, false).await
}
