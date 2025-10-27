use crate::structs::database::{guilds::Guild, oauth::OauthToken, reminders::Reminder, tags::Tag};
use mongodb::Collection;

pub mod guilds;
pub mod oauth;
pub mod redis;
pub mod reminders;
pub mod tags;

pub struct Collections {
    pub guilds: Collection<Guild>,
    pub oauth: Collection<OauthToken>,
    pub reminders: Collection<Reminder>,
    pub tags: Collection<Tag>,
}
