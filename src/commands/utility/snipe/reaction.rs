use crate::structs::{command_context::CommandContext, snipes::ReactionSnipes};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let guild_id = ctx.input.guild_id.as_ref().unwrap();

    let message = ctx.get_string_arg("message")?;
    let mut split = message.split('/').rev();
    let (message_id, channel_id) = (split.next().unwrap(), split.next().unwrap_or(ctx.input.channel_id.as_ref().unwrap()));

    let permissions = ctx.input.app_permissions;

    match ReactionSnipes::new(guild_id, channel_id, message_id, permissions).to_response() {
        Ok(response) => ctx.respond(response, false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
