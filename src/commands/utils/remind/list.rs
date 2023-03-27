use crate::{
    macros::{format_timestamp, if_else},
    statics::{colors::PRIMARY_COLOR, emojis::ERROR_EMOJI, MONGODB},
    structs::{duration::Duration, reminders::Reminder},
};
use anyhow::Result;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::embeds::Embed,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    res.defer(false).await?;

    let reminders = MONGODB.get().unwrap().collection::<Reminder>("reminders");

    let entries = reminders
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
                format_timestamp!(reminder.timestamp),
                if_else!(
                    reminder.interval > 0,
                    format!(" (every {})", Duration::new().parse(reminder.interval).unwrap()),
                    "".into()
                )
            )
        })
        .collect::<Vec<String>>();

    if_else!(
        entries.is_empty(),
        res.send_message(format!("{ERROR_EMOJI} No reminders found.")).await?,
        res.send_message(
            Embed::new()
                .set_color(PRIMARY_COLOR)?
                .set_description(entries.join("\n\n"))
        )
        .await?
    );

    Ok(())
}
