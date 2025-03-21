use crate::structs::{command_context::CommandContext, database::guilds::Guilds};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let mut guild = Guilds::get(ctx.input.guild_id.as_ref().unwrap()).await?;

    if let Ok(fix_embeds) = ctx.get_bool_arg("fix-embeds") {
        guild.fix_embeds = fix_embeds;
    }

    match Guilds::update(guild).await {
        Ok(_) => ctx.respond_success("Updated.", true).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
