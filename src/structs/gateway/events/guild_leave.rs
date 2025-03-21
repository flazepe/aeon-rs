use crate::structs::{database::guilds::Guilds, gateway::events::handler::EventHandler};
use anyhow::Result;
use twilight_model::gateway::payload::incoming::GuildDelete;

impl EventHandler {
    pub async fn on_guild_delete(guild: GuildDelete) -> Result<()> {
        if guild.unavailable.is_none() {
            Guilds::delete(guild.id).await?;
        }

        Ok(())
    }
}
