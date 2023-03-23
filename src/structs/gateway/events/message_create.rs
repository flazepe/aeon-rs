use crate::{statics::*, structs::gateway::events::handler::EventHandler, traits::*};
use twilight_model::gateway::payload::incoming::MessageCreate;

impl EventHandler {
    pub fn on_message_create(message: Box<MessageCreate>) {
        let message = message.0;

        if message.guild_id.is_none() {
            return;
        }

        let mut channels = CACHE.channels.lock().unwrap();
        let channel_id = message.channel_id.to_string();

        if !channels.contains_key(&channel_id) {
            channels.insert(channel_id.clone(), vec![]);
        }

        channels
            .get_mut(&channel_id)
            .unwrap()
            .push_limited(message, 50);
    }
}
