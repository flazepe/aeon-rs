use crate::{
    functions::add_reminder_select_options,
    macros::if_else,
    statics::{colors::NOTICE_COLOR, duration::SECS_PER_MONTH, CONFIG, MONGODB},
    structs::duration::Duration,
};
use anyhow::{bail, Result};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use slashook::{
    commands::MessageResponse,
    rest::Rest,
    structs::{
        channels::{Channel, Message},
        components::{Components, SelectMenu, SelectMenuType},
        embeds::Embed,
    },
};
use std::{
    thread::sleep,
    time::{Duration as TimeDuration, SystemTime, UNIX_EPOCH},
};

#[derive(Deserialize, Serialize)]
pub struct Reminder {
    pub _id: ObjectId,
    pub user_id: String,
    pub url: String,
    pub timestamp: u64,
    pub interval: u64,
    pub reminder: String,
    pub dm: bool,
}

pub struct Reminders {
    rest: Option<Rest>,
    reminders: Collection<Reminder>,
}

impl Reminders {
    pub fn new() -> Self {
        Self {
            rest: None,
            reminders: MONGODB.get().unwrap().collection::<Reminder>("reminders"),
        }
    }

    pub async fn poll(mut self) -> Result<()> {
        loop {
            let current_timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

            for mut reminder in self
                .reminders
                .find(doc! { "timestamp": { "$lte": current_timestamp as i64 } }, None)
                .await?
                .try_collect::<Vec<Reminder>>()
                .await?
            {
                match self.handle(&reminder).await {
                    Ok(_) => {
                        self.reminders.delete_one(doc! { "_id": reminder._id }, None).await?;

                        if reminder.interval > 0 {
                            // To prevent spam and keeping precision, while loop is needed to ensure that the new timestamp isn't behind the current timestamp
                            while reminder.timestamp <= current_timestamp {
                                reminder.timestamp += reminder.interval;
                            }

                            self.reminders.insert_one(&reminder, None).await?;
                        }
                    },
                    Err(error) => {
                        let error = error.to_string();

                        println!(
                            "[REMINDERS] An error occurred while handling reminder {}: {error}",
                            reminder._id
                        );

                        if let Some(fatal_error) = ["Invalid Recipient(s)", "Missing Access", "Unknown Channel"]
                            .iter()
                            .find(|message| error.contains(&message.to_string()))
                        {
                            self.reminders.delete_one(doc! { "_id": reminder._id }, None).await?;

                            println!(
                                "[REMINDERS] Deleted reminder {} due to fatal error \"{fatal_error}\".",
                                reminder._id
                            );
                        }
                    },
                }
            }

            sleep(TimeDuration::from_secs(10));
        }
    }

    async fn handle(&mut self, reminder: &Reminder) -> Result<()> {
        if self.rest.is_none() {
            self.rest = Some(Rest::with_token(CONFIG.bot.token.clone()));
        }

        let mut response = MessageResponse::from(if_else!(reminder.dm, "".into(), format!("<@{}>", reminder.user_id)))
            .add_embed(
                Embed::new()
                    .set_color(NOTICE_COLOR)?
                    .set_title("Reminder")
                    .set_url(format!("https://discord.com/channels/{}", reminder.url))
                    .set_description(&reminder.reminder),
            );

        if reminder.interval == 0 {
            response = response.set_components(
                Components::new().add_select_menu(
                    add_reminder_select_options(SelectMenu::new(SelectMenuType::STRING))
                        .set_id("remind", reminder.url.clone())
                        .set_placeholder("Snooze"),
                ),
            );
        }

        Message::create(
            self.rest.as_ref().unwrap(),
            if reminder.dm && !reminder.url.contains("@me") {
                // If the reminder should be DM'd but was created inside a guild channel, we have to create a new DM channel
                self.rest
                    .as_ref()
                    .unwrap()
                    .post::<Channel, _>("users/@me/channels".into(), json!({ "recipient_id": reminder.user_id }))
                    .await?
                    .id
            } else {
                // Else, just grab channel ID from the URL
                reminder.url.split("/").skip(1).next().unwrap().to_string()
            },
            response,
        )
        .await?;

        Ok(())
    }

    pub async fn get_many<T: ToString>(&self, user_id: T) -> Result<Vec<Reminder>> {
        let reminders = self
            .reminders
            .find(
                doc! {
                    "user_id": user_id.to_string(),
                },
                None,
            )
            .await?
            .try_collect::<Vec<Reminder>>()
            .await?;

        if_else!(reminders.is_empty(), bail!("No reminders found."), Ok(reminders))
    }

    pub async fn set<T: ToString, U: ToString, V: ToString>(
        &self,
        user_id: T,
        url: U,
        time: Duration,
        interval: Duration,
        reminder: V,
        dm: bool,
    ) -> Result<String> {
        if self
            .reminders
            .count_documents(doc! { "user_id": user_id.to_string() }, None)
            .await?
            >= 10
        {
            bail!("You can only have up to 10 reminders.");
        };

        if time.total_secs < 30 || time.total_secs > SECS_PER_MONTH * 12 {
            bail!("Time cannot be under 30 seconds or over a year.");
        }

        if interval.total_secs > 0 && (interval.total_secs < 30 || interval.total_secs > SECS_PER_MONTH * 12) {
            bail!("Interval cannot be under 30 seconds or over a year.");
        }

        if interval.total_secs > 0 && !dm {
            bail!("Intervals are only supported for DMs.");
        }

        // For older snooze messages
        if !url.to_string().contains("/") {
            bail!("Unsupported message.");
        }

        self.reminders
            .insert_one(
                &Reminder {
                    _id: ObjectId::new(),
                    user_id: user_id.to_string(),
                    url: url.to_string(),
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + time.total_secs,
                    interval: interval.total_secs,
                    reminder: reminder.to_string(),
                    dm,
                },
                None,
            )
            .await?;

        Ok(format!(
            "I will remind you about [{}](<https://discord.com/channels/{}>) in {time}{}. Make sure I {}.",
            reminder.to_string(),
            url.to_string(),
            if_else!(
                interval.total_secs > 0,
                format!(" and every {interval} after that"),
                "".into(),
            ),
            if_else!(dm, "can DM you", "have the View Channel and Send Messages permission"),
        ))
    }

    pub async fn delete<T: ToString>(&self, id: T) -> Result<()> {
        self.reminders.delete_one(doc! { "_id": id.to_string() }, None).await?;
        Ok(())
    }
}
