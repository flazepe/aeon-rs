use crate::{statics::CACHE, traits::LimitedVec};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::MessageDelete;

pub async fn handle(event: &MessageDelete) -> Result<()> {
    let mut channels = CACHE.channels.write().unwrap();
    let channel_id = event.channel_id.to_string();

    if !channels.contains_key(&channel_id) {
        channels.insert(channel_id.clone(), vec![]);
    }

    let messages = channels.get_mut(&channel_id).unwrap();

    if let Some(entry) = messages.iter().enumerate().find(|(_, message)| message.id == event.id) {
        let message = messages.remove(entry.0);

        // Snipes
        let mut channels = CACHE.snipes.write().unwrap();

        if !channels.contains_key(&channel_id) {
            channels.insert(channel_id.clone(), vec![]);
        }

        channels.get_mut(&channel_id).unwrap().push_limited(message, 50);
    }

    Ok(())
}
