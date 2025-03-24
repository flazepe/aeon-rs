use crate::structs::command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt};
use anyhow::Result;
use serde_json::json;
use slashook::structs::guilds::GuildMember;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let user = input.get_user_arg("member")?;

    match input
        .rest
        .patch::<GuildMember, _>(
            format!("guilds/{}/members/{}", input.guild_id.as_ref().unwrap(), user.id),
            json!({ "communication_disabled_until": null }),
        )
        .await
    {
        Ok(_) => ctx.respond_success(format!("Removed timeout for {}.", user.mention()), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
