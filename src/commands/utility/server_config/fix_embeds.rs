use crate::{
    statics::MONGODB,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    let mongodb = MONGODB.get().unwrap();
    let mut guild = mongodb.guilds.get(input.guild_id.as_ref().unwrap()).await?;

    if let Ok(enabled) = ctx.get_bool_arg("enabled") {
        guild.fix_embeds.enabled = enabled;
    }

    let message = if guild.fix_embeds.enabled { "Enabled `fix-embeds`." } else { "Disabled `fix-embeds`." };

    mongodb.guilds.update(guild).await?;
    ctx.respond_success(message, true).await
}
