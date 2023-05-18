use crate::{
    functions::{format_timestamp, TimestampFormat},
    statics::colors::PRIMARY_COLOR,
    structs::{command_context::CommandContext, database::reminders::Reminders, duration::Duration},
};
use anyhow::Result;
use slashook::structs::embeds::Embed;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Reminders::new().get_many(&ctx.input.user.id).await {
        Ok(reminders) => {
            ctx.respond(
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
        Err(error) => ctx.respond_error(error, true).await,
    }
}
