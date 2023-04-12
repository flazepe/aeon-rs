use crate::{
    statics::{
        duration::SECS_PER_DAY,
        emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
    },
    structs::duration::Duration,
    traits::ArgGetters,
};
use anyhow::Result;
use serde_json::json;
use slashook::{
    chrono::{Duration as ChronoDuration, Utc},
    commands::{CommandInput, CommandResponder},
    structs::guilds::GuildMember,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    match Duration::new().parse(input.get_string_arg("duration")?) {
        Ok(duration) => {
            if duration.total_secs < 30 || duration.total_secs > SECS_PER_DAY * 28 {
                res.send_message(format!(
                    "{ERROR_EMOJI} Time cannot be under 30 seconds or over 28 days."
                ))
                .await?;

                return Ok(());
            }

            let user = input.get_user_arg("member")?;

            match input
                .rest
                .patch::<GuildMember, _>(
                    format!("guilds/{}/members/{}", input.guild_id.as_ref().unwrap(), &user.id),
                    json!({
                        "communication_disabled_until": (Utc::now()
                            + ChronoDuration::seconds(duration.total_secs as i64)).to_rfc3339()
                    }),
                )
                .await
            {
                Ok(_) => {
                    res.send_message(format!(
                        "{SUCCESS_EMOJI} Set timeout for {} for {duration}.",
                        user.mention()
                    ))
                    .await?;
                },
                Err(error) => {
                    res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                },
            }
        },
        Err(error) => {
            res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
        },
    }

    Ok(())
}
