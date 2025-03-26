use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput},
    duration::{Duration, statics::SECS_PER_DAY},
};
use anyhow::{Result, bail};
use serde_json::json;
use slashook::{
    chrono::{Duration as ChronoDuration, Utc},
    structs::guilds::GuildMember,
};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

    let duration = Duration::new().parse(ctx.get_string_arg("duration")?)?;

    if duration.total_secs < 30 || duration.total_secs > SECS_PER_DAY * 28 {
        bail!("Duration cannot be under 30 seconds or over 28 days.");
    }

    let user = ctx.get_user_arg("member")?;

    input
        .rest
        .patch::<GuildMember, _>(
            format!("guilds/{}/members/{}", input.guild_id.as_ref().unwrap(), user.id),
            json!({ "communication_disabled_until": (Utc::now() + ChronoDuration::try_seconds(duration.total_secs as i64).unwrap()).to_rfc3339() }),
        )
        .await?;

    ctx.respond_success(format!("Set timeout for {} for {duration}.", user.mention()), false).await
}
