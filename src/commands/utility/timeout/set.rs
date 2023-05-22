use crate::structs::{
    command_context::CommandContext,
    duration::{statics::SECS_PER_DAY, Duration},
};
use anyhow::Result;
use serde_json::json;
use slashook::{
    chrono::{Duration as ChronoDuration, Utc},
    structs::guilds::GuildMember,
};

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Duration::new().parse(ctx.get_string_arg("duration")?) {
        Ok(duration) => {
            if duration.total_secs < 30 || duration.total_secs > SECS_PER_DAY * 28 {
                return ctx.respond_error("Time cannot be under 30 seconds or over 28 days.", true).await;
            }

            let user = ctx.get_user_arg("member")?;

            match ctx
                .input
                .rest
                .patch::<GuildMember, _>(
                    format!("guilds/{}/members/{}", ctx.input.guild_id.as_ref().unwrap(), user.id),
                    json!({ "communication_disabled_until": (Utc::now() + ChronoDuration::seconds(duration.total_secs as i64)).to_rfc3339() }),
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
