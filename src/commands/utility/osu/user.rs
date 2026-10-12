use crate::structs::{
    api::osu::Osu,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(..) = &ctx.command_input else { return Ok(()) };

    let mode = ctx.get_string_arg("mode", 0, true);
    let user = Osu::get_user(ctx.get_string_arg("user", 0, true)?, mode.as_deref().unwrap_or("default")).await?;

    ctx.respond_pagination(user.format_pagination("osu"), false).await
}
