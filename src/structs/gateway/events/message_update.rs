use crate::{statics::CACHE, structs::gateway::events::handler::EventHandler, traits::LimitedVec};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::MessageUpdate;

impl EventHandler {
    pub async fn on_message_update(message: Box<MessageUpdate>) -> Result<()> {
        let message = message.0;

        let mut channels = CACHE.channels.write().unwrap();
        let channel_id = message.channel_id.to_string();

        if !channels.contains_key(&channel_id) {
            channels.insert(channel_id.clone(), vec![]);
        }

        let old_message = channels.get_mut(&channel_id).unwrap().iter_mut().find(|_message| _message.id == message.id);

        if let Some(old_message) = old_message {
            let cloned_old_message = old_message.clone();

            // Update message
            old_message.attachments = message.attachments;
            old_message.content = message.content;
            old_message.edited_timestamp = message.edited_timestamp;
            old_message.embeds = message.embeds;
            old_message.pinned = message.pinned;

            // Edit snipes
            let mut channels = CACHE.edit_snipes.write().unwrap();

            if !channels.contains_key(&channel_id) {
                channels.insert(channel_id.clone(), vec![]);
            }

            channels.get_mut(&channel_id).unwrap().push_limited(cloned_old_message, 50);
        }

        Ok(())
    }
}
