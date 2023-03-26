use crate::{statics::CACHE, structs::gateway::events::handler::EventHandler, traits::LimitedVec};
use twilight_model::gateway::payload::incoming::MessageDelete;

impl EventHandler {
    pub fn on_message_delete(message: MessageDelete) {
        let mut channels = CACHE.channels.lock().unwrap();
        let channel_id = message.channel_id.to_string();

        if !channels.contains_key(&channel_id) {
            channels.insert(channel_id.clone(), vec![]);
        }

        let messages = channels.get_mut(&channel_id).unwrap();

        if let Some(entry) = messages
            .iter()
            .enumerate()
            .find(|(_, _message)| _message.id == message.id)
        {
            let message = messages.remove(entry.0);

            // Snipes
            let mut channels = CACHE.snipes.lock().unwrap();

            if !channels.contains_key(&channel_id) {
                channels.insert(channel_id.clone(), vec![]);
            }

            channels.get_mut(&channel_id).unwrap().push_limited(message, 50);
        }
    }
}
