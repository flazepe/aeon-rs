use crate::structs::database::{oauth::OauthToken, reminders::Reminder, tags::Tag};
use mongodb::Collection;

pub mod oauth;
pub mod reminders;
pub mod tags;

pub struct Collections {
    pub oauth: Collection<OauthToken>,
    pub reminders: Collection<Reminder>,
    pub tags: Collection<Tag>,
}
