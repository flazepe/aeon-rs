use crate::{
    macros::if_else,
    statics::{colors::NOTICE_COLOR, CONFIG, MONGODB},
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
        components::{Components, SelectMenu, SelectMenuType, SelectOption},
        embeds::Embed,
    },
};
use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
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
    rest: Rest,
    reminders: Collection<Reminder>,
}

impl Reminders {
    pub fn new() -> Self {
        Self {
            rest: Rest::with_token(CONFIG.bot.token.clone()),
            reminders: MONGODB.get().unwrap().collection::<Reminder>("reminders"),
        }
    }

    pub async fn poll(self) -> Result<()> {
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
                    Ok(()) => {
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
                        println!(
                            "[REMINDERS] An error occurred while handling reminder {}: {error}",
                            reminder._id
                        );
                    },
                }
            }

            sleep(Duration::from_secs(10));
        }
    }

    async fn handle(&self, reminder: &Reminder) -> Result<()> {
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
                    SelectMenu::new(SelectMenuType::STRING)
                        .set_id("remind", "time")
                        .set_placeholder("Snooze")
                        .add_option(SelectOption::new("5 minutes", "5m"))
                        .add_option(SelectOption::new("15 minutes", "15m"))
                        .add_option(SelectOption::new("30 minutes", "30m"))
                        .add_option(SelectOption::new("1 hour", "1h"))
                        .add_option(SelectOption::new("3 hours", "3h"))
                        .add_option(SelectOption::new("6 hours", "6h"))
                        .add_option(SelectOption::new("12 hours", "12h"))
                        .add_option(SelectOption::new("24 hours", "24h")),
                ),
            );
        }

        Message::create(
            &self.rest,
            if reminder.dm && !reminder.url.contains("@me") {
                // If the reminder should be DM'd but was created inside a guild channel, we have to create a new DM channel
                self.create_dm_channel(reminder).await?.id
            } else {
                // Else, just grab channel ID from the URL
                reminder.url.split("/").skip(1).next().unwrap().to_string()
            },
            response,
        )
        .await?;

        Ok(())
    }

    async fn create_dm_channel(&self, reminder: &Reminder) -> Result<Channel> {
        match self
            .rest
            .post::<Channel, _>("users/@me/channels".into(), json!({ "recipient_id": reminder.user_id }))
            .await
        {
            Ok(channel) => Ok(channel),
            Err(error) => {
                let error = error.to_string();

                if reminder.interval > 0
                    && ["Invalid Recipient(s)", "Missing Access", "Unknown Channel"]
                        .iter()
                        .any(|message| error.contains(message))
                {
                    self.reminders.delete_one(doc! { "_id": reminder._id }, None).await?;
                }

                bail!("Could not create DM channel.");
            },
        }
    }
}
