use crate::{statics::*, structs::gateway::events::handler::EventHandler};
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

impl EventHandler {
    pub fn on_message_delete_bulk(data: MessageDeleteBulk) {
        let mut cache = CACHE.lock().unwrap();

        let channels = &mut cache.channels;
        let channel_id = data.channel_id.to_string();

        if let Some(messages) = channels.get_mut(&channel_id) {
            let mut deleted_messages = vec![];

            for index in messages
                .iter()
                .filter(|message| data.ids.contains(&message.id))
                .enumerate()
                .map(|(index, _)| index)
                .collect::<Vec<usize>>()
            {
                deleted_messages.push(messages.remove(index));
            }

            let snipe_channels = &mut cache.snipes;

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
