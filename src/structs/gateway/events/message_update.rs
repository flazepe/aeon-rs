use crate::structs::gateway::cache::CACHE;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub fn handle(message: Box<MessageUpdate>) {
    let mut cache = CACHE.lock().unwrap();
    let channels = &mut cache.channels;
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
        let channels = &mut cache.edit_snipes;

        if !channels.contains_key(&channel_id) {
            channels.insert(channel_id.clone(), vec![]);
        }

        let edit_snipes = channels.get_mut(&channel_id).unwrap();

        edit_snipes.push(cloned_old_message);

        // Limit
        if edit_snipes.len() > 50 {
            let edit_snipes = channels.remove(&channel_id).unwrap();
            let skip_amount = edit_snipes.len() - 50;

            channels.insert(
                channel_id,
                edit_snipes.into_iter().skip(skip_amount).collect(),
            );
        }
    }
}
