use crate::structs::{command_context::CommandContext, database::guilds::Guilds};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let mut guild = Guilds::get(ctx.input.guild_id.as_ref().unwrap()).await?;
    guild.fix_embeds = ctx.get_bool_arg("enabled")?;

    let message = if guild.fix_embeds { "Enabled `fix-embeds`." } else { "Disabled `fix-embeds`." };

    if let Err(error) = Guilds::update(guild).await {
        ctx.respond_error(error, true).await
    } else {
        ctx.respond_success(message, true).await
    }
}
