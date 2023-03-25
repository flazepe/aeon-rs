use crate::{statics::*, structs::gateway::events::handler::EventHandler};
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

impl EventHandler {
    pub fn on_message_delete_bulk(data: MessageDeleteBulk) {
        let channel_id = data.channel_id.to_string();

        if let Some(messages) = CACHE.channels.lock().unwrap().get_mut(&channel_id) {
            let mut deleted_messages = vec![];

            for id in data.ids {
                if let Some((index, _)) = messages
                    .iter()
                    .enumerate()
                    .find(|(_, message)| message.id == id)
                {
                    deleted_messages.push(messages.remove(index));
                }
            }

            let mut snipe_channels = CACHE.snipes.lock().unwrap();

            if !snipe_channels.contains_key(&channel_id) {
                snipe_channels.insert(channel_id.clone(), vec![]);
            }

            snipe_channels
                .get_mut(&channel_id)
                .unwrap()
                .append(&mut deleted_messages);
        }
    }
}
