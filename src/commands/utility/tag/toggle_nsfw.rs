use crate::{
    statics::MONGODB,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let name = ctx.get_string_arg("tag", 0, true)?;
    let guild_id = input.guild_id.as_ref().unwrap();
    let modifier = input.member.as_ref().unwrap();

    let mongodb = MONGODB.get().unwrap();
    let response = mongodb.tags.toggle_nsfw(name, guild_id, modifier).await?;

    ctx.respond_success(response, true).await
}
