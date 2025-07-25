use crate::{
    functions::{add_reminder_select_options, now},
    statics::{COLLECTIONS, REST, colors::NOTICE_EMBED_COLOR},
    structs::{
        client::AeonClient,
        duration::{Duration, statics::SECS_PER_MONTH},
    },
};
use anyhow::{Result, bail};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_json::json;
use slashook::{
    commands::MessageResponse,
    structs::{
        channels::Channel,
        components::{Components, SelectMenu, SelectMenuType},
        embeds::Embed,
        messages::Message,
    },
};
use std::{fmt::Display, thread::sleep, time::Duration as TimeDuration};

#[derive(Deserialize, Serialize, Debug)]
pub struct Reminder {
    pub _id: ObjectId,
    pub user_id: String,
    pub url: String,
    pub timestamp: u64,
    pub interval: u64,
    pub reminder: String,
    pub dm: bool,
}

pub struct Reminders;

impl Reminders {
    pub async fn poll() -> Result<()> {
        // Use a separate client for polling
        let reminders = AeonClient::connect_to_database().await?.collection("reminders");

        loop {
            let current_timestamp = now();

            for mut reminder in
                reminders.find(doc! { "timestamp": { "$lte": current_timestamp as i64 } }).await?.try_collect::<Vec<Reminder>>().await?
            {
                match Self::handle(&reminder).await {
                    Ok(_) => {
                        reminders.delete_one(doc! { "_id": reminder._id }).await?;

                        if reminder.interval > 0 {
                            // To prevent spam and keeping precision, while loop is needed to ensure that the new timestamp isn't behind the current timestamp
                            while reminder.timestamp <= current_timestamp {
                                reminder.timestamp += reminder.interval;
                            }

                            reminders.insert_one(&reminder).await?;
                        }
                    },
                    Err(error) => {
                        let error = error.to_string();

                        println!("[REMINDERS] An error occurred while handling reminder {}: {error}", reminder._id);

                        if let Some(fatal_error) = [
                            "Cannot send messages to this user",
                            "Invalid Recipient(s)",
                            "Missing Access",
                            "Missing Permissions",
                            "Unknown Channel",
                        ]
                        .iter()
                        .find(|message| error.contains(&message.to_string()))
                        {
                            reminders.delete_one(doc! { "_id": reminder._id }).await?;
                            println!(r#"[REMINDERS] Deleted reminder {} due to fatal error "{fatal_error}"."#, reminder._id);
                        }
                    },
                }
            }

            sleep(TimeDuration::from_secs(10));
        }
    }

    async fn handle(reminder: &Reminder) -> Result<()> {
        let mention: String = if reminder.dm { "".into() } else { format!("<@{}>", reminder.user_id) };

        let mut response = MessageResponse::from(mention).add_embed(
            Embed::new()
                .set_color(NOTICE_EMBED_COLOR)?
                .set_title("Reminder")
                .set_url(format!("https://discord.com/channels/{}", reminder.url))
                .set_description(&reminder.reminder),
        );

        if reminder.interval == 0 {
            response = response.set_components(
                Components::new().add_select_menu(
                    add_reminder_select_options(SelectMenu::new(SelectMenuType::STRING))
                        .set_id("reminder", reminder.url.clone())
                        .set_placeholder("Snooze"),
                ),
            );
        }

        let channel_id = if reminder.dm {
            // If the reminder should be DM'd, we have to create a new DM channel
            REST.post::<Channel, _>("users/@me/channels".into(), json!({ "recipient_id": reminder.user_id })).await?.id
        } else {
            // Else, just grab channel ID from the URL
            reminder.url.split('/').nth(1).unwrap().to_string()
        };

        Message::create(&REST, channel_id, response).await?;

        Ok(())
    }

    pub async fn get_many<T: Display>(user_id: T) -> Result<Vec<Reminder>> {
        let reminders = COLLECTIONS
            .reminders
            .find(doc! { "user_id": user_id.to_string() })
            .sort(doc! { "timestamp": 1 })
            .await?
            .try_collect::<Vec<Reminder>>()
            .await?;

        if reminders.is_empty() {
            bail!("No reminders found.");
        }

        Ok(reminders)
    }

    pub async fn set<T: Display, U: Display, V: Display>(
        user_id: T,
        url: U,
        duration: Duration,
        interval: Duration,
        reminder: V,
        dm: bool,
    ) -> Result<String> {
        if COLLECTIONS.reminders.count_documents(doc! { "user_id": user_id.to_string() }).await? >= 15 {
            bail!("You can only have up to 15 reminders.");
        }

        if duration.total_secs < 30 || duration.total_secs > SECS_PER_MONTH * 12 {
            bail!("Duration cannot be under 30 seconds or over a year.");
        }

        if interval.total_secs > 0 && (interval.total_secs < 30 || interval.total_secs > SECS_PER_MONTH * 12) {
            bail!("Interval cannot be under 30 seconds or over a year.");
        }

        if interval.total_secs > 0 && !dm {
            bail!("Intervals are only supported for DMs.");
        }

        // For older snooze messages
        if !url.to_string().contains('/') {
            bail!("Unsupported message.");
        }

        COLLECTIONS
            .reminders
            .insert_one(&Reminder {
                _id: ObjectId::new(),
                user_id: user_id.to_string(),
                url: url.to_string(),
                timestamp: now() + duration.total_secs,
                interval: interval.total_secs,
                reminder: reminder.to_string(),
                dm,
            })
            .await?;

        Ok(format!(
            "I will remind you about `{}`[*](<https://discord.com/channels/{url}>) in {duration}{}. Make sure I {}.",
            reminder.to_string().replace('`', "｀"),
            if interval.total_secs > 0 { format!(" and every {interval} after that") } else { "".into() },
            if dm { "can DM you" } else { "have the View Channel and Send Messages permission" },
        ))
    }

    pub async fn delete(id: ObjectId) -> Result<()> {
        COLLECTIONS.reminders.delete_one(doc! { "_id": id }).await?;
        Ok(())
    }
}
