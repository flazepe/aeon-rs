use crate::structs::{
    command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
    database::tags::Tags,
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };
    let name = input.get_string_arg("tag")?;
    let guild_id = input.guild_id.as_ref().unwrap();
    let modifier = input.member.as_ref().unwrap();

    match Tags::toggle_nsfw(name, guild_id, modifier).await {
        Ok(response) => ctx.respond_success(response, true).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
