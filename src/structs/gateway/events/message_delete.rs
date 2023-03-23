use crate::{statics::*, structs::gateway::events::handler::EventHandler, traits::*};
use twilight_model::gateway::payload::incoming::MessageDelete;

impl EventHandler {
    pub fn on_message_delete(message: MessageDelete) {
        let mut cache = CACHE.lock().unwrap();
        let channels = &mut cache.channels;
        let channel_id = message.channel_id.to_string();

        if !channels.contains_key(&channel_id) {
            channels.insert(channel_id.clone(), vec![]);
        }

        let messages = channels.get_mut(&channel_id).unwrap();

        let entry = messages
            .iter()
            .enumerate()
            .find(|(_, _message)| _message.id == message.id);

        if let Some(entry) = entry {
            let message = messages.remove(entry.0);

            // Snipes
            let channels = &mut cache.snipes;

            if !channels.contains_key(&channel_id) {
                channels.insert(channel_id.clone(), vec![]);
            }

            channels
                .get_mut(&channel_id)
                .unwrap()
                .push_limited(message, 50);
        }
    }
}
