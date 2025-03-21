use crate::structs::{database::guilds::Guilds, gateway::events::handler::EventHandler};
use twilight_model::gateway::payload::incoming::GuildDelete;

impl EventHandler {
    pub async fn on_guild_delete(guild: GuildDelete) {
        if guild.unavailable.is_none() {
            let _ = Guilds::delete(guild.id).await;
        }
    }
}
