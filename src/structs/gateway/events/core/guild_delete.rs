use crate::structs::database::guilds::Guilds;
use anyhow::Result;
use twilight_model::gateway::payload::incoming::GuildDelete;

pub async fn handle(event: &GuildDelete) -> Result<()> {
    if event.unavailable.is_none() {
        Guilds::delete(event.id).await?;
    }

    Ok(())
}
