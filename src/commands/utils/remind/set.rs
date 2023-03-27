use crate::{
    macros::{and_then_or, if_else},
    statics::{
        duration::SECS_PER_MONTH,
        emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
        MONGODB,
    },
    structs::{duration::Duration, reminders::Reminder},
    traits::ArgGetters,
};
use anyhow::Result;
use mongodb::bson::{doc, oid::ObjectId};
use slashook::commands::{CommandInput, CommandResponder};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    res.defer(false).await?;

    let reminders = MONGODB.get().unwrap().collection::<Reminder>("reminders");

    if reminders
        .count_documents(doc! { "user_id": input.user.id.to_string() }, None)
        .await?
        >= 10
    {
        res.send_message(format!("{ERROR_EMOJI} You can only have up to 10 reminders."))
            .await?;

        return Ok(());
    };

    let time = Duration::new()
        .parse(and_then_or!(
            input.values.as_ref(),
            |values| Some(values[0].to_string()),
            input.get_string_arg("time").unwrap_or("".into())
        ))
        .unwrap_or(Duration::new());

    let interval = Duration::new()
        .parse(input.get_string_arg("interval").unwrap_or("".into()))
        .unwrap_or(Duration::new());

    if (time.total_secs < 30 || time.total_secs > SECS_PER_MONTH * 12)
        || (interval.total_secs > 0 && (interval.total_secs < 30 || interval.total_secs > SECS_PER_MONTH * 12))
    {
        res.send_message(format!(
            "{ERROR_EMOJI} Time or interval cannot be under 30 seconds or over a year."
        ))
        .await?;

        return Ok(());
    }

    let dm = input.guild_id.is_none() || input.get_bool_arg("dm")?;

    if interval.total_secs > 0 && !dm {
        res.send_message(format!("{ERROR_EMOJI} Intervals are only supported for DMs."))
            .await?;

        return Ok(());
    }

    let reminder = {
        let mut reminder = input.get_string_arg("reminder").unwrap_or("Do something".into());

        if input.values.is_some() {
            if let Some(parsed_reminder) =
                || -> Option<&String> { input.message.as_ref()?.embeds.get(0)?.description.as_ref() }()
            {
                reminder = parsed_reminder.to_string();
            };
        }

        reminder
    };

    let url = {
        let empty_string = String::new();
        let custom_id = input.custom_id.as_ref().unwrap_or(&empty_string);

        if_else!(
            custom_id.contains('/'),
            custom_id.to_string(),
            format!(
                "{}/{}/{}",
                input.guild_id.as_ref().unwrap_or(&"@me".into()),
                input.channel_id.as_ref().unwrap(),
                res.get_original_message().await?.id
            )
        )
    };

    res.send_message(format!(
        "{SUCCESS_EMOJI} I will remind you about [{reminder}](<https://discord.com/channels/{url}>) in {time}{}. Make sure I {}.",
        if_else!(
            interval.total_secs > 0,
            format!(" and every {interval} after that"),
            "".into()
        ),
        if_else!(dm, "can DM you", "have the View Channel and Send Messages permission")
    ))
    .await?;

    reminders
        .insert_one(
            &Reminder {
                _id: ObjectId::new(),
                user_id: input.user.id.to_string(),
                url,
                timestamp: (SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + time.total_secs),
                interval: interval.total_secs,
                reminder: reminder.to_string(),
                dm,
            },
            None,
        )
        .await?;

    Ok(())
}
