use crate::structs::{
    command_context::{CommandContext, CommandInputExt, Input},
    database::guilds::Guilds,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };
    let mut guild = Guilds::get(input.guild_id.as_ref().unwrap()).await?;
    guild.logs_channel_id = input.get_channel_arg("channel").ok().map(|channel| channel.id.clone());

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
