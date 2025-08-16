use crate::{statics::CACHE, traits::LimitedVec};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle(event: &MessageCreate) -> Result<()> {
    {
        let mut channels = CACHE.discord.channels.write().unwrap();
        let channel_id = event.channel_id.to_string();

        if !channels.contains_key(&channel_id) {
            channels.insert(channel_id.clone(), vec![]);
        }

        channels.get_mut(&channel_id).unwrap().push_limited(event.0.clone(), 50);
    }

    Ok(())
}
