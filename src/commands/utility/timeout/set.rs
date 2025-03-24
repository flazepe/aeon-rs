use crate::structs::{
    command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
    duration::{Duration, statics::SECS_PER_DAY},
};
use anyhow::Result;
use serde_json::json;
use slashook::{
    chrono::{Duration as ChronoDuration, Utc},
    structs::guilds::GuildMember,
};

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };

    match Duration::new().parse(input.get_string_arg("duration")?) {
        Ok(duration) => {
            if duration.total_secs < 30 || duration.total_secs > SECS_PER_DAY * 28 {
                return ctx.respond_error("Duration cannot be under 30 seconds or over 28 days.", true).await;
            }

            let user = input.get_user_arg("member")?;

            match input
                .rest
                .patch::<GuildMember, _>(
                    format!("guilds/{}/members/{}", input.guild_id.as_ref().unwrap(), user.id),
                    json!({ "communication_disabled_until": (Utc::now() + ChronoDuration::try_seconds(duration.total_secs as i64).unwrap()).to_rfc3339() }),
                )
                .await
            {
                Ok(_) => ctx.respond_success(format!("Set timeout for {} for {duration}.", user.mention()), false).await,
                Err(error) => ctx.respond_error(error, true).await,
            }
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}
