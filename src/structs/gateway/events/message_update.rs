use crate::{statics::*, structs::gateway::events::handler::EventHandler, traits::*};
use twilight_model::gateway::payload::incoming::MessageUpdate;

impl EventHandler {
    pub fn on_message_update(message: Box<MessageUpdate>) {
        let mut channels = CACHE.channels.lock().unwrap();
        let channel_id = message.channel_id.to_string();

        if !channels.contains_key(&channel_id) {
            channels.insert(channel_id.clone(), vec![]);
        }

        let old_message = channels
            .get_mut(&channel_id)
            .unwrap()
            .iter_mut()
            .find(|_message| _message.id == message.id);

        if let Some(old_message) = old_message {
            let cloned_old_message = old_message.clone();

            // Update message
            if let Some(attachments) = message.attachments {
                old_message.attachments = attachments;
            }

            if let Some(content) = message.content {
                old_message.content = content;
            }

            old_message.edited_timestamp = message.edited_timestamp;

            if let Some(embeds) = message.embeds {
                old_message.embeds = embeds;
            }

            old_message.pinned = message.pinned.unwrap_or(old_message.pinned);

            // Edit snipes
            let mut channels = CACHE.edit_snipes.lock().unwrap();

            if !channels.contains_key(&channel_id) {
                channels.insert(channel_id.clone(), vec![]);
            }

            channels
                .get_mut(&channel_id)
                .unwrap()
                .push_limited(cloned_old_message, 50);
        }
    }
}
