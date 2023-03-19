use crate::structs::gateway::cache::CACHE;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub fn handle(message: Box<MessageCreate>) {
    let message = message.0;

    let mut cache = CACHE.lock().unwrap();
    let channels = &mut cache.channels;
    let channel_id = message.channel_id.to_string();

    if !channels.contains_key(&channel_id) {
        channels.insert(channel_id.clone(), vec![]);
    }

    let messages = channels.get_mut(&channel_id).unwrap();

    messages.push(message);

    // Limit
    if messages.len() > 50 {
        let messages = channels.remove(&channel_id).unwrap();
        let skip_amount = messages.len() - 50;

        channels.insert(channel_id, messages.into_iter().skip(skip_amount).collect());
    }
}
