use crate::structs::{
    command_context::{CommandContext, CommandInputExt, Input},
    snipes::ReactionSnipes,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand(input,  _) = &ctx.input else { return Ok(()) };
    let guild_id = input.guild_id.as_ref().unwrap();

    let message = input.get_string_arg("message")?;
    let mut split = message.split('/').rev();
    let (message_id, channel_id) = (split.next().unwrap(), split.next().unwrap_or(input.channel_id.as_ref().unwrap()));

    let permissions = input.app_permissions;

    match ReactionSnipes::new(guild_id, channel_id, message_id, permissions).to_response() {
        Ok(response) => ctx.respond(response, false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
