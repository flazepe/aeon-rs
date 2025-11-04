use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput},
    database::Database,
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    let mongodb = Database::get_mongodb()?;
    let mut guild = mongodb.guilds.get(input.guild_id.as_ref().unwrap()).await?;

    if let Ok(enabled) = ctx.get_bool_arg("enabled") {
        guild.logs.enabled = enabled;
    }

    if let Ok(channel) = ctx.get_channel_arg("channel") {
        guild.logs.channel_id = Some(channel.id.clone());
    }

    if let Ok(ignore_bots) = ctx.get_bool_arg("ignore-bots") {
        guild.logs.ignore_bots = ignore_bots;
    }

    let message = format!(
        "Updated logs config ({}, {}, {}).",
        if guild.logs.enabled { "enabled" } else { "disabled" },
        guild.logs.channel_id.as_ref().map(|logs_channel_id| format!("<#{logs_channel_id}>")).as_deref().unwrap_or("no channel set"),
        if guild.logs.ignore_bots { "ignore bots" } else { "don't ignore bots" },
    );

    mongodb.guilds.update(guild).await?;
    ctx.respond_success(message, true).await
}
