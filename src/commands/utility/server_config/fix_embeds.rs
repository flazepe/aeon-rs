use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput},
    database::guilds::Guilds,
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let mut guild = Guilds::get(input.guild_id.as_ref().unwrap()).await?;

    if let Ok(enabled) = ctx.get_bool_arg("enabled") {
        guild.fix_embeds.enabled = enabled;
    }

    let message = if guild.fix_embeds.enabled { "Enabled `fix-embeds`." } else { "Disabled `fix-embeds`." };

    Guilds::update(guild).await?;
    ctx.respond_success(message, true).await
}
