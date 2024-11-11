use crate::{
    functions::{format_timestamp, TimestampFormat},
    statics::colors::PRIMARY_COLOR,
    structs::{command_context::CommandContext, database::reminders::Reminders, duration::Duration},
};
use anyhow::Result;
use slashook::structs::embeds::Embed;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match Reminders::get_many(&ctx.input.user.id).await {
        Ok(reminders) => {
            let description = reminders
                .iter()
                .enumerate()
                .map(|(index, reminder)| {
                    format!(
                        "{}. `{}`[*](https://discord.com/channels/{})\n{}{}",
                        index + 1,
                        reminder.reminder.replace('`', "｀"),
                        reminder.url,
                        format_timestamp(reminder.timestamp, TimestampFormat::Full),
                        if reminder.interval > 0 {
                            format!(" (every {})", Duration::new().parse(reminder.interval).unwrap_or_default())
                        } else {
                            "".into()
                        },
                    )
                })
                .collect::<Vec<String>>()
                .join("\n\n");
            let embed = Embed::new().set_color(PRIMARY_COLOR)?.set_description(description);

            ctx.respond(embed, true).await
        },
        Err(error) => ctx.respond_error(error, true).await,
    }
}
