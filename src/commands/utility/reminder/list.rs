use crate::{
    functions::{TimestampFormat, format_timestamp},
    statics::colors::PRIMARY_COLOR,
    structs::{
        command_context::{AeonCommandContext, AeonCommandInput},
        database::reminders::Reminders,
        duration::Duration,
    },
};
use anyhow::Result;
use slashook::structs::embeds::Embed;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };

    match Reminders::get_many(&input.user.id).await {
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
