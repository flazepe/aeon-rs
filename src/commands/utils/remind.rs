use crate::{
    statics::{colors::*, duration::*, emojis::*},
    structs::{duration::Duration, reminders::*},
    traits::*,
    *,
};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, to_document};
use slashook::{
    command,
    commands::*,
    structs::{embeds::Embed, interactions::*},
};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_command() -> Command {
    #[command(
        name = "remind",
        description = "Manages your reminders.",
        subcommands = [
            {
                name = "set",
                description = "Sets a reminder.",
                options = [
                    {
                        name = "time",
                        description = "The duration to remind, e.g. 1h",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
                    {
                        name = "reminder",
                        description = "The reminder",
                        option_type = InteractionOptionType::STRING,
						max_length = 300,
                    },
                    {
                        name = "interval",
                        description = "The interval time to remind, e.g. 1h",
                        option_type = InteractionOptionType::STRING
                    },
					{
                        name = "dm",
                        description = "Whether to DM instead",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
                ],
            },
            {
                name = "list",
                description = "Sends the reminder list.",
            },
			{
                name = "delete",
                description = "Deletes a reminder.",
                options = [
                    {
                        name = "entry",
                        description = "The reminder entry. Can be provided by using the list subcommand",
                        option_type = InteractionOptionType::INTEGER,
						min_value = 1.0,
                        required = true,
                    },
                ],
            },
        ],
    )]
    fn remind(input: CommandInput, res: CommandResponder) {
        res.defer(false).await?;

        // Snooze
        if input.custom_id == Some("time".into()) {
            return set_reminder(&input, &res, true).await?;
        };

        let reminders = MONGODB.get().unwrap().collection::<Reminder>("reminders");

        match input.subcommand.as_deref().unwrap_or("") {
            "set" => {
                set_reminder(&input, &res, false).await?;
            }
            "list" => {
                let mut cursor = reminders
                    .find(doc! { "user_id": input.user.id.to_string() }, None)
                    .await?;

                let entries = {
                    let mut entries = vec![];

                    while let Some(reminder) = cursor.try_next().await? {
                        entries.push(format!(
                            "{}. {}\n{}{}",
                            entries.len() + 1,
                            reminder.reminder,
                            format_timestamp!(reminder.timestamp),
                            if_else!(
                                reminder.interval > 0,
                                format!(" (every {})", Duration::new().parse(reminder.interval)?),
                                "".into()
                            )
                        ));
                    }

                    entries
                };

                if_else!(
                    entries.is_empty(),
                    res.send_message(format!("{ERROR_EMOJI} No reminders found."))
                        .await?,
                    res.send_message(
                        Embed::new()
                            .set_color(PRIMARY_COLOR)?
                            .set_description(entries.join("\n\n"))
                    )
                    .await?
                );
            }
            "delete" => {
                let mut cursor = reminders
                    .find(doc! { "user_id": input.user.id.to_string() }, None)
                    .await?;

                match {
                    let mut entries = vec![];

                    while let Some(reminder) = cursor.try_next().await? {
                        entries.push(reminder);
                    }

                    entries
                }
                .get(input.get_i64_arg("entry")? as usize - 1)
                {
                    Some(entry) => {
                        reminders
                            .delete_one(doc! { "_id": entry._id }, None)
                            .await?;

                        res.send_message(format!("{SUCCESS_EMOJI} Gone.")).await?
                    }
                    None => {
                        res.send_message(format!(
                            "{ERROR_EMOJI} Invalid entry. Make sure it's a valid number."
                        ))
                        .await?
                    }
                }
            }
            _ => {}
        }
    }

    remind
}

pub async fn set_reminder(
    input: &CommandInput,
    res: &CommandResponder,
    snooze: bool,
) -> Result<()> {
    let reminders = MONGODB.get().unwrap().collection::<Reminder>("reminders");

    if reminders
        .count_documents(doc! { "user_id": input.user.id.to_string() }, None)
        .await?
        >= 10
    {
        res.send_message(format!(
            "{ERROR_EMOJI} You can only have up to 10 reminders."
        ))
        .await?;

        return Ok(());
    };

    let mut reminder = input
        .get_string_arg("reminder")
        .unwrap_or("Do something".into());
    let time;
    let interval = input.get_string_arg("interval").unwrap_or("".into());
    let mut dm = input.get_bool_arg("dm")?;

    if snooze {
        if let Some(parsed_reminder) =
            || -> Option<&String> { input.message.as_ref()?.embeds.get(0)?.description.as_ref() }()
        {
            reminder = parsed_reminder.to_string();
        }
        time = input.values.as_ref().unwrap()[0].to_string();
        dm = input.guild_id.is_none();
    } else {
        time = input.get_string_arg("time")?;
    }

    let time = Duration::new().parse(time).unwrap_or(Duration::new());
    let interval = Duration::new().parse(interval).unwrap_or(Duration::new());

    if (time.total_secs < 30 || time.total_secs > SECS_PER_MONTH * 12)
        || (interval.total_secs > 0
            && (interval.total_secs < 30 || interval.total_secs > SECS_PER_MONTH * 12))
    {
        res.send_message(format!(
            "{ERROR_EMOJI} Time or interval cannot be under 30 seconds or over a year."
        ))
        .await?;

        return Ok(());
    }

    res.send_message(format!(
        "{SUCCESS_EMOJI} I will remind you about \"{reminder}\" in {time}{}. Make sure I {}.",
        if_else!(
            interval.total_secs > 0,
            format!(" and every {interval} after that"),
            "".into()
        ),
        if_else!(
            dm,
            "can DM you",
            "have the View Channel and Send Messages permission"
        )
    ))
    .await?;

    // This dumb to_document() only returns a Document so we need an untyped collection
    MONGODB
        .get()
        .unwrap()
        .collection("reminders")
        .insert_one(
            to_document(&Reminder {
                _id: ObjectId::new(),
                user_id: input.user.id.to_string(),
                url: {
                    let guild_id = and_then_or!(
                        input.guild_id.as_ref(),
                        |guild_id| Some(guild_id.to_string()),
                        "".into()
                    );

                    if !dm && !guild_id.is_empty() {
                        Some(format!(
                            "{guild_id}/{}/{}",
                            input.channel_id.as_ref().unwrap(),
                            res.get_original_message().await?.id
                        ))
                    } else {
                        None
                    }
                },
                timestamp: (SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
                    + time.total_secs),
                interval: interval.total_secs,
                reminder: reminder.to_string(),
            })?,
            None,
        )
        .await?;

    Ok(())
}