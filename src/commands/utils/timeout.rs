use crate::{constants::*, structs::duration::*, traits::*};
use anyhow::Context;
use serde_json::json;
use slashook::{
    chrono::{Duration as ChronoDuration, Utc},
    command,
    commands::*,
    structs::{guilds::GuildMember, interactions::*, Permissions},
};

pub fn get_command() -> Command {
    #[command(
		name = "timeout",
		description = "Manages members' timeout.",
		default_member_permissions = Permissions::MODERATE_MEMBERS,
		dm_permission = false,
		subcommands = [
			{
				name = "set",
				description = "Sets a member's timeout.",
				options = [
					{
						name = "member",
						description = "The member",
						option_type = InteractionOptionType::USER,
						required = true,
					},
					{
						name = "duration",
						description = "The duration to timeout, e.g. 1h",
						option_type = InteractionOptionType::STRING,
						required = true,
					},
				],
			},
			{
				name = "remove",
				description = "Removes a member's timeout.",
				options = [
					{
						name = "member",
						description = "The member",
						option_type = InteractionOptionType::USER,
						required = true,
					},
				],
			},
		],
	)]
    async fn timeout(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref().unwrap_or("") {
            "set" => match Duration::new().parse(input.get_string_arg("duration")?) {
                Ok(duration) => {
                    if duration.total_secs < 30 || duration.total_secs > SECS_PER_DAY * 28 {
                        return res
                            .send_message(format!(
                                "{ERROR_EMOJI} time cannot be under 30 seconds or over 28 days"
                            ))
                            .await?;
                    }

                    let user = input.get_user_arg("member")?;

                    match input
                        .rest
                        .patch::<GuildMember, _>(
                            format!("guilds/{}/members/{}", input.guild_id.as_ref().context("missing guild ID")?, &user.id),
                            json!({
                                "communication_disabled_until": (Utc::now()
                                    + ChronoDuration::seconds(duration.total_secs as i64)).to_rfc3339()
                            }),
                        )
                        .await
                    {
                        Ok(_) => {
							res.send_message(format!("set timeout for <@{}> for {duration}", user.id)).await?;
						}
                        Err(error) => {
                            res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                        }
                    }
                }
                Err(error) => {
                    res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                }
            },
            "remove" => {
                let user = input.get_user_arg("member")?;

                match input
                    .rest
                    .patch::<GuildMember, _>(
                        format!(
                            "guilds/{}/members/{}",
                            input.guild_id.as_ref().context("missing guild ID")?,
                            &user.id
                        ),
                        json!({ "communication_disabled_until": null }),
                    )
                    .await
                {
                    Ok(_) => {
                        res.send_message(format!("removed timeout for <@{}>", user.id))
                            .await?;
                    }
                    Err(error) => {
                        res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                    }
                }
            }
            _ => {}
        }
    }

    timeout
}
