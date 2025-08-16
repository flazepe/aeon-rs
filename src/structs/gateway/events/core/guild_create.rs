use crate::statics::CACHE;
use anyhow::Result;
use twilight_model::gateway::payload::incoming::GuildCreate;

pub async fn handle(event: &GuildCreate) -> Result<()> {
    let GuildCreate::Available(event) = event else { return Ok(()) };
    CACHE.discord_guilds.write().unwrap().insert(event.id.to_string(), event.clone());
    Ok(())
}
