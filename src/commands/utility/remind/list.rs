use crate::{
    functions::{format_timestamp, TimestampFormat},
    macros::if_else,
    statics::{colors::PRIMARY_COLOR, emojis::ERROR_EMOJI, MONGODB},
    structs::{duration::Duration, reminders::Reminder},
};
use anyhow::Result;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::embeds::Embed,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let entries = MONGODB
        .get()
        .unwrap()
        .collection::<Reminder>("reminders")
        .find(doc! { "user_id": input.user.id.to_string() }, None)
        .await?
        .try_collect::<Vec<Reminder>>()
        .await?
        .iter()
        .enumerate()
        .map(|(index, reminder)| {
            format!(
                "{}. [{}](https://discord.com/channels/{})\n{}{}",
                index + 1,
                reminder.reminder,
                reminder.url,
                format_timestamp(reminder.timestamp, TimestampFormat::Full),
                if_else!(
                    reminder.interval > 0,
                    format!(" (every {})", Duration::new().parse(reminder.interval).unwrap()),
                    "".into(),
                ),
            )
        })
        .collect::<Vec<String>>();

    res.send_message(
        if_else!(
            entries.is_empty(),
            MessageResponse::from(format!("{ERROR_EMOJI} No reminders found.")),
            MessageResponse::from(
                Embed::new()
                    .set_color(PRIMARY_COLOR)?
                    .set_description(entries.join("\n\n"))
            )
        )
        .set_ephemeral(true),
    )
    .await?;

    Ok(())
}
