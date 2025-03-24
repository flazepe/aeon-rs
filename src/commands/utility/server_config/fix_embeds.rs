use crate::structs::{
    command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
    database::guilds::Guilds,
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };
    let mut guild = Guilds::get(input.guild_id.as_ref().unwrap()).await?;
    guild.fix_embeds = input.get_bool_arg("enabled")?;

    let message = if guild.fix_embeds { "Enabled `fix-embeds`." } else { "Disabled `fix-embeds`." };

    if let Err(error) = Guilds::update(guild).await {
        ctx.respond_error(error, true).await
    } else {
        ctx.respond_success(message, true).await
    }
}
