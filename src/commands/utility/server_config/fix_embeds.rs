use crate::structs::{
    command_context::{CommandContext, CommandInputExt, Input},
    database::guilds::Guilds,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand(input,  _) = &ctx.input else { return Ok(()) };
    let mut guild = Guilds::get(input.guild_id.as_ref().unwrap()).await?;
    guild.fix_embeds = input.get_bool_arg("enabled")?;

    let message = if guild.fix_embeds { "Enabled `fix-embeds`." } else { "Disabled `fix-embeds`." };

    if let Err(error) = Guilds::update(guild).await {
        ctx.respond_error(error, true).await
    } else {
        ctx.respond_success(message, true).await
    }
}
