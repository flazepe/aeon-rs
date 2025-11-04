use crate::{statics::CACHE, structs::database::Database};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::GuildDelete;

pub async fn handle(event: &GuildDelete) -> Result<()> {
    if event.unavailable.is_none() {
        let mongodb = Database::get_mongodb()?;
        mongodb.guilds.delete(event.id).await?;
        CACHE.discord.guilds.write().unwrap().remove(&event.id.to_string());
    }

    Ok(())
}
