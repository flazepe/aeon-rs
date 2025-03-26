use crate::structs::command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt};
use anyhow::Result;
use serde_json::json;
use slashook::structs::guilds::GuildMember;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let user = input.get_user_arg("member")?;

    input
        .rest
        .patch::<GuildMember, _>(
            format!("guilds/{}/members/{}", input.guild_id.as_ref().unwrap(), user.id),
            json!({ "communication_disabled_until": null }),
        )
        .await?;

    ctx.respond_success(format!("Removed timeout for {}.", user.mention()), false).await
}
