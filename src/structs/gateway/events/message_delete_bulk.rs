use crate::{statics::CACHE, structs::gateway::events::handler::EventHandler};
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

impl EventHandler {
    pub async fn on_message_delete_bulk(data: MessageDeleteBulk) {
        let channel_id = data.channel_id.to_string();

        if let Some(messages) = CACHE.channels.write().unwrap().get_mut(&channel_id) {
            let mut deleted_messages = vec![];

            for id in data.ids {
                if let Some((index, _)) = messages.iter().enumerate().find(|(_, message)| message.id == id) {
                    deleted_messages.push(messages.remove(index));
                }
            }

            let mut channels = CACHE.snipes.write().unwrap();

            if !channels.contains_key(&channel_id) {
                channels.insert(channel_id.clone(), vec![]);
            }

            channels.get_mut(&channel_id).unwrap().append(&mut deleted_messages);
        }
    }
}
