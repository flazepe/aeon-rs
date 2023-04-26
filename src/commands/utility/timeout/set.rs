use crate::{
    structs::{
        duration::{statics::SECS_PER_DAY, Duration},
        interaction::Interaction,
    },
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
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    match Duration::new().parse(input.get_string_arg("duration")?) {
        Ok(duration) => {
            if duration.total_secs < 30 || duration.total_secs > SECS_PER_DAY * 28 {
                return interaction.respond_error("Time cannot be under 30 seconds or over 28 days.", true).await;
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
                Ok(_) => interaction.respond_success(format!("Set timeout for {} for {duration}.", user.mention()), false).await,
                Err(error) => interaction.respond_error(error, true).await,
            }
        },
        Err(error) => interaction.respond_error(error, true).await,
    }
}
