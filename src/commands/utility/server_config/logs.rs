use crate::structs::{command_context::CommandContext, database::guilds::Guilds};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let mut guild = Guilds::get(ctx.input.guild_id.as_ref().unwrap()).await?;
    guild.logs_channel_id = ctx.get_channel_arg("channel").ok().map(|channel| channel.id.clone());

    let message = if let Some(logs_channel_id) = &guild.logs_channel_id {
        format!("Enabled logs (<#{logs_channel_id}>).")
    } else {
        "Disabled logs.".into()
    };

    if let Err(error) = Guilds::update(guild).await {
        ctx.respond_error(error, true).await
    } else {
        ctx.respond_success(message, true).await
    }
}
