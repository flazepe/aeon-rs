use crate::structs::database::{oauth::OAuthToken, reminders::Reminder, tags::Tag};
use mongodb::Collection;

pub mod oauth;
pub mod reminders;
pub mod tags;

pub struct Collections {
    pub oauth: Collection<OAuthToken>,
    pub reminders: Collection<Reminder>,
    pub tags: Collection<Tag>,
}
