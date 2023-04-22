use crate::{
    macros::if_else,
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
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    res.defer(input.is_string_select()).await?;

    if let Some(message) = input.message.as_ref() {
        // Delete snoozed reminder
        if message.interaction.is_none() {
            input
                .rest
                .delete::<()>(format!("channels/{}/messages/{}", message.channel_id, message.id))
                .await
                .ok();
        }
    }

    let reminders = MONGODB.get().unwrap().collection::<Reminder>("reminders");

    if reminders
        .count_documents(doc! { "user_id": input.user.id.to_string() }, None)
        .await?
        >= 10
    {
        res.send_message(
            MessageResponse::from(format!("{ERROR_EMOJI} You can only have up to 10 reminders.")).set_ephemeral(true),
        )
        .await?;

        return Ok(());
    };

    let time = Duration::new()
        .parse(input.values.as_ref().map_or_else(
            || input.get_string_arg("time").unwrap_or("".into()),
            |values| values[0].to_string(),
        ))
        .unwrap_or(Duration::new());

    let interval = Duration::new()
        .parse(input.get_string_arg("interval").unwrap_or("".into()))
        .unwrap_or(Duration::new());

    if (time.total_secs < 30 || time.total_secs > SECS_PER_MONTH * 12)
        || (interval.total_secs > 0 && (interval.total_secs < 30 || interval.total_secs > SECS_PER_MONTH * 12))
    {
        res.send_message(
            MessageResponse::from(format!(
                "{ERROR_EMOJI} Time or interval cannot be under 30 seconds or over a year.",
            ))
            .set_ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    let dm = input.message.as_ref().map_or(
        false,
        // DM if select menu's message was from an interaction
        |message| message.interaction.is_some(),
    ) || input.guild_id.is_none()
        || input.get_bool_arg("dm").unwrap_or(false);

    if interval.total_secs > 0 && !dm {
        res.send_message(
            MessageResponse::from(format!("{ERROR_EMOJI} Intervals are only supported for DMs.")).set_ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    let reminder = {
        let mut reminder = input.get_string_arg("reminder").unwrap_or("Do something".into());

        if input.is_string_select() {
            if let Some(parsed_reminder) =
                || -> Option<&String> { input.message.as_ref()?.embeds.get(0)?.description.as_ref() }()
            {
                reminder = parsed_reminder.to_string();
            };
        }

        reminder
    };

    let original_message = res.get_original_message().await?;

    let url = input.custom_id.as_ref().map_or_else(
        || {
            format!(
                "{}/{}/{}",
                input.guild_id.as_ref().unwrap_or(&"@me".into()),
                input.channel_id.as_ref().unwrap(),
                original_message.id,
            )
        },
        |custom_id| custom_id.to_string(),
    );

    // For older snooze messages
    if !url.contains('/') {
        res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} Unsupported message.")).set_ephemeral(true))
            .await?;

        return Ok(());
    }

    reminders
        .insert_one(
            &Reminder {
                _id: ObjectId::new(),
                user_id: input.user.id.clone(),
                url: url.clone(),
                timestamp: (SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + time.total_secs),
                interval: interval.total_secs,
                reminder: reminder.clone(),
                dm,
            },
            None,
        )
        .await?;

    res.send_message(MessageResponse::from(format!(
        "{SUCCESS_EMOJI} I will remind you about [{reminder}](<https://discord.com/channels/{url}>) in {time}{}. Make sure I {}.",
        if_else!(
            interval.total_secs > 0,
            format!(" and every {interval} after that"),
            "".into(),
        ),
        if_else!(dm, "can DM you", "have the View Channel and Send Messages permission"),
    )).set_suppress_embeds(true)).await?;

    Ok(())
}
