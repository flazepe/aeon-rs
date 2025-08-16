use crate::{statics::CACHE, traits::LimitedVec};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn handle(event: &MessageUpdate) -> Result<()> {
    let mut channels = CACHE.discord.channels.write().unwrap();
    let channel_id = event.channel_id.to_string();

    if !channels.contains_key(&channel_id) {
        channels.insert(channel_id.clone(), vec![]);
    }

    let old_message = channels.get_mut(&channel_id).unwrap().iter_mut().find(|message| message.id == event.id);

    if let Some(old_message) = old_message {
        let cloned_old_message = old_message.clone();

        // Update message
        let new_message = event.0.clone();

        old_message.attachments = new_message.attachments;
        old_message.content = new_message.content;
        old_message.edited_timestamp = new_message.edited_timestamp;
        old_message.embeds = new_message.embeds;
        old_message.pinned = new_message.pinned;

        // Add edit snipe
        let mut channels = CACHE.discord.edit_snipes.write().unwrap();

        if !channels.contains_key(&channel_id) {
            channels.insert(channel_id.clone(), vec![]);
        }

        channels.get_mut(&channel_id).unwrap().push_limited(cloned_old_message, 50);
    }

    // Update embed fix response if available
    if let Some(embed_fix_response) = CACHE.discord.embed_fix_responses.write().unwrap().get_mut(&event.id.to_string()) {
        embed_fix_response.content = event.content.clone();
    }

    Ok(())
}
