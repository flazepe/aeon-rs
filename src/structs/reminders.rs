use crate::{
    statics::{colors::*, CONFIG, MONGODB},
    *,
};
use anyhow::{bail, Context, Result};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    Collection,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use slashook::{
    commands::MessageResponse,
    rest::Rest,
    structs::{channels::*, components::*, embeds::Embed},
};
use std::{
    thread::sleep,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize)]
pub struct Reminder {
    pub _id: ObjectId,
    pub user_id: String,
    pub url: Option<String>,
    pub timestamp: u64,
    pub interval: u64,
    pub reminder: String,
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
            let mut cursor = self.reminders.find(doc! {}, None).await?;

            while let Some(mut reminder) = cursor.try_next().await? {
                if SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() >= reminder.timestamp {
                    match self.handle(&reminder).await {
                        Ok(()) => {
                            if reminder.interval > 0 {
                                reminder.timestamp = reminder.timestamp + reminder.interval;

                                self.reminders
                                    .update_one(
                                        doc! { "_id": reminder._id },
                                        to_document(&reminder)?,
                                        None,
                                    )
                                    .await?;
                            } else {
                                self.reminders
                                    .delete_one(doc! { "_id": reminder._id }, None)
                                    .await?;
                            }
                        }
                        Err(error) => {
                            println!("{error}");
                        }
                    }
                }
            }

            sleep(Duration::from_secs(10));
        }
    }

    async fn handle(&self, reminder: &Reminder) -> Result<()> {
        let mut message_url = String::from("https://discord.com");

        Message::create(
            &self.rest,
            if let Some(url) = reminder.url.as_ref() {
                message_url += &format!("/channels/{url}");
                url.split("/").skip(1).next().unwrap().to_string()
            } else {
                match self
                    .rest
                    .post::<Channel, _>(
                        "users/@me/channels".into(),
                        json!({ "recipient_id": reminder.user_id }),
                    )
                    .await
                {
                    Ok(channel) => channel.id,
                    Err(error) => {
                        let error = error.to_string();

                        if reminder.interval > 0
                            && ["Invalid Recipient(s)", "Missing Access", "Unknown Channel"]
                                .iter()
                                .any(|message| error.contains(message))
                        {
                            self.reminders
                                .delete_one(doc! { "_id": reminder._id }, None)
                                .await?;
                        }

                        bail!("Could not create DM channel.");
                    }
                }
            },
            MessageResponse::from(if_else!(
                reminder.url.is_some(),
                format!("<@{}>", reminder.user_id),
                "".into()
            ))
            .add_embed(
                Embed::new()
                    .set_color(NOTICE_COLOR)?
                    .set_title("Reminder")
                    .set_url(message_url)
                    .set_description(&reminder.reminder),
            )
            .set_components(
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
            ),
        )
        .await?;

        Ok(())
    }
}
