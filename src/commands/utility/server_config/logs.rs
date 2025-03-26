use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    database::guilds::Guilds,
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let mut guild = Guilds::get(input.guild_id.as_ref().unwrap()).await?;
    guild.logs_channel_id = input.get_channel_arg("channel").ok().map(|channel| channel.id.clone());

    let message = if let Some(logs_channel_id) = &guild.logs_channel_id {
        format!("Enabled logs (<#{logs_channel_id}>).")
    } else {
        "Disabled logs.".into()
    };

    Guilds::update(guild).await?;
    ctx.respond_success(message, true).await
}
