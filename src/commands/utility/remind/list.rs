use crate::{
    functions::{format_timestamp, TimestampFormat},
    statics::colors::PRIMARY_COLOR,
    structs::{database::reminders::Reminders, duration::Duration, interaction::Interaction},
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::embeds::Embed,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    match Reminders::new().get_many(&input.user.id).await {
        Ok(reminders) => {
            interaction
                .respond(
                    Embed::new().set_color(PRIMARY_COLOR)?.set_description(
                        reminders
                            .iter()
                            .enumerate()
                            .map(|(index, reminder)| {
                                format!(
                                    "{}. [{}](https://discord.com/channels/{})\n{}{}",
                                    index + 1,
                                    reminder.reminder,
                                    reminder.url,
                                    format_timestamp(reminder.timestamp, TimestampFormat::Full),
                                    match reminder.interval > 0 {
                                        true => format!(" (every {})", Duration::new().parse(reminder.interval).unwrap()),
                                        false => "".into(),
                                    },
                                )
                            })
                            .collect::<Vec<String>>()
                            .join("\n\n"),
                    ),
                    true,
                )
                .await
        },
        Err(error) => interaction.respond_error(error, true).await,
    }
}
