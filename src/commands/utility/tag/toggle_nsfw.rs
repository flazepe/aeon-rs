use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    database::tags::Tags,
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let name = input.get_string_arg("tag")?;
    let guild_id = input.guild_id.as_ref().unwrap();
    let modifier = input.member.as_ref().unwrap();
    let response = Tags::toggle_nsfw(name, guild_id, modifier).await?;

    ctx.respond_success(response, true).await
}
