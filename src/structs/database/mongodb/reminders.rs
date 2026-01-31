use crate::{
    functions::add_reminder_select_options,
    statics::{REST, colors::NOTICE_EMBED_COLOR},
    structs::{
        database::mongodb::MongoDB,
        duration::{Duration, statics::SECS_PER_MONTH},
    },
};
use anyhow::{Result, bail};
use futures::{StreamExt, stream::TryStreamExt};
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use slashook::{
    chrono::Utc,
    commands::MessageResponse,
    structs::{
        channels::Channel,
        components::{Components, SelectMenu, SelectMenuType},
        embeds::Embed,
        messages::Message,
    },
};
use std::{fmt::Display, thread::sleep, time::Duration as TimeDuration};
use tracing::{error, warn};

const DISCORD_API_FATAL_ERRORS: [&str; 5] =
    ["Cannot send messages to this user", "Invalid Recipient(s)", "Missing Access", "Missing Permissions", "Unknown Channel"];

#[derive(Deserialize, Serialize, Debug)]
pub struct Reminder {
    pub _id: ObjectId,
    pub user_id: String,
    pub url: String,
    pub timestamp: i64,
    pub interval: i64,
    pub reminder: String,
    pub dm: bool,
}

#[derive(Debug)]
pub struct Reminders {
    collection: Collection<Reminder>,
}

impl Reminders {
    pub fn new(collection: Collection<Reminder>) -> Self {
        Self { collection }
    }

    pub async fn poll() -> Result<()> {
        // Use a separate connection for polling
        let collection = MongoDB::get_database().await?.collection::<Reminder>("reminders");

        loop {
            let current_timestamp = Utc::now().timestamp();
            let mut reminders = collection.find(doc! { "timestamp": { "$lte": current_timestamp } }).await?;

            while let Some(result) = reminders.next().await {
                let mut reminder = match result {
                    Ok(reminder) => reminder,
                    Err(error) => {
                        error!(target = "Reminders", "An error occurred while parsing reminder: {error:#?}");
                        continue;
                    },
                };

                if let Err(error) = Self::handle(&reminder).await {
                    let reminder_id = reminder._id;
                    let error = format!("{error:#?}");

                    error!(target: "Reminders", "An error occurred while handling reminder {reminder_id}: {error}");

                    if let Some(message) = DISCORD_API_FATAL_ERRORS.iter().find(|message| error.contains(&message.to_string())) {
                        collection.delete_one(doc! { "_id": reminder_id }).await?;
                        warn!(target: "Reminders", r#"Deleted reminder {reminder_id} due to fatal error "{message}"."#);
                    }
                } else {
                    collection.delete_one(doc! { "_id": reminder._id }).await?;

                    if reminder.interval > 0 {
                        // To prevent spam and keeping precision, while loop is needed to ensure that the new timestamp isn't behind the current timestamp
                        while reminder.timestamp <= current_timestamp {
                            reminder.timestamp += reminder.interval;
                        }

                        collection.insert_one(&reminder).await?;
                    }
                }
            }

            sleep(TimeDuration::from_secs(10));
        }
    }

    async fn handle(reminder: &Reminder) -> Result<()> {
        let user_id = &reminder.user_id;
        let user_mention = if reminder.dm { "".into() } else { format!("<@{user_id}>") };
        let url = &reminder.url;

        let mut response = MessageResponse::from(user_mention).add_embed(
            Embed::new()
                .set_color(NOTICE_EMBED_COLOR)?
                .set_title("Reminder")
                .set_url(format!("https://discord.com/channels/{url}"))
                .set_description(&reminder.reminder),
        );

        if reminder.interval == 0 {
            response = response.set_components(
                Components::new().add_select_menu(
                    add_reminder_select_options(SelectMenu::new(SelectMenuType::STRING))
                        .set_id("reminder", url.clone())
                        .set_placeholder("Snooze"),
                ),
            );
        }

        let channel_id = if reminder.dm {
            // If the reminder should be DM'd, we have to create a new DM channel
            REST.post::<Channel, _>("users/@me/channels".into(), json!({ "recipient_id": user_id })).await?.id
        } else {
            // Else, just grab channel ID from the URL
            reminder.url.split('/').nth(1).unwrap().to_string()
        };

        Message::create(&REST, channel_id, response).await?;

        Ok(())
    }

    pub async fn get_many<T: Display>(&self, user_id: T) -> Result<Vec<Reminder>> {
        let reminders = self
            .collection
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
        &self,
        user_id: T,
        url: U,
        duration: Duration,
        interval: Duration,
        reminder: V,
        dm: bool,
    ) -> Result<String> {
        if self.collection.count_documents(doc! { "user_id": user_id.to_string() }).await? >= 15 {
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

        self.collection
            .insert_one(&Reminder {
                _id: ObjectId::new(),
                user_id: user_id.to_string(),
                url: url.to_string(),
                timestamp: Utc::now().timestamp() + duration.total_secs as i64,
                interval: interval.total_secs as i64,
                reminder: reminder.to_string(),
                dm,
            })
            .await?;

        Ok(format!(
            "I will remind you about ``{}``[*](<https://discord.com/channels/{url}>) in {duration}{}. Make sure I {}.",
            reminder.to_string().replace('`', "`\u{200b}"),
            if interval.total_secs > 0 { format!(" and every {interval} after that") } else { "".into() },
            if dm { "can DM you" } else { "have the View Channel and Send Messages permission" },
        ))
    }

    pub async fn delete(&self, id: ObjectId) -> Result<()> {
        self.collection.delete_one(doc! { "_id": id }).await?;
        Ok(())
    }
}
