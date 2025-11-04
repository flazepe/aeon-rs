use crate::statics::CONFIG;
use anyhow::Result;
use guilds::Guilds;
use mongodb::{Client, Database};
use oauth::OAuth;
use reminders::Reminders;
use tags::Tags;

pub mod guilds;
pub mod oauth;
pub mod reminders;
pub mod tags;

#[derive(Debug)]
pub struct MongoDB {
    pub guilds: Guilds,
    pub oauth: OAuth,
    pub reminders: Reminders,
    pub tags: Tags,
}

impl MongoDB {
    pub async fn get_database() -> Result<Database> {
        Ok(Client::with_uri_str(&CONFIG.database.mongodb_uri).await?.database("aeon"))
    }

    pub async fn new() -> Result<Self> {
        let database = Self::get_database().await?;

        let guilds = Guilds::new(database.collection("guilds"));
        let oauth = OAuth::new(database.collection("oauth"));
        let reminders = Reminders::new(database.collection("reminders"));
        let tags = Tags::new(database.collection("tags"));

        Ok(Self { guilds, oauth, reminders, tags })
    }
}
