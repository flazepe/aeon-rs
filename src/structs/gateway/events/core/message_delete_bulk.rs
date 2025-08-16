use crate::statics::CACHE;
use anyhow::Result;
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

pub async fn handle(event: &MessageDeleteBulk) -> Result<()> {
    let channel_id = event.channel_id.to_string();

    if let Some(messages) = CACHE.discord.channels.write().unwrap().get_mut(&channel_id) {
        let mut deleted_messages = vec![];

        for id in &event.ids {
            if let Some((index, _)) = messages.iter().enumerate().find(|(_, message)| &message.id == id) {
                deleted_messages.push(messages.remove(index));
            }
        }

        let mut channels = CACHE.discord.snipes.write().unwrap();

        if !channels.contains_key(&channel_id) {
            channels.insert(channel_id.clone(), vec![]);
        }

        channels.get_mut(&channel_id).unwrap().append(&mut deleted_messages);
    }

    Ok(())
}
