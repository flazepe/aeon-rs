use crate::structs::gateway::cache::CACHE;
use twilight_model::gateway::payload::incoming::MessageDelete;

pub fn handle(message: MessageDelete) {
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

        let snipes = channels.get_mut(&channel_id).unwrap();

        snipes.push(message);

        // Limit
        if snipes.len() > 50 {
            let snipes = channels.remove(&channel_id).unwrap();
            let skip_amount = snipes.len() - 50;

            channels.insert(channel_id, snipes.into_iter().skip(skip_amount).collect());
        }
    }
}
