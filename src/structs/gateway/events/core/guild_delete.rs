use crate::{statics::CACHE, structs::database::guilds::Guilds};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::GuildDelete;

pub async fn handle(event: &GuildDelete) -> Result<()> {
    if event.unavailable.is_none() {
        Guilds::delete(event.id).await?;
        CACHE.discord_guilds.write().unwrap().remove(&event.id.to_string());
    }

    Ok(())
}
