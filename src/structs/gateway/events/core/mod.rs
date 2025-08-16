mod guild_create;
mod guild_delete;
mod guild_update;
mod member_add;
mod member_remove;
mod message_create;
mod message_delete;
mod message_delete_bulk;
mod message_update;
mod presence_update;
mod reaction_add;
mod reaction_remove;

use crate::structs::gateway::events::EventHandler;
use anyhow::Result;
use twilight_gateway::Event;

impl EventHandler {
    pub async fn handle_core(event: &Event) -> Result<()> {
        match event {
            Event::GuildCreate(event) => guild_create::handle(event).await,
            Event::GuildDelete(event) => guild_delete::handle(event).await,
            Event::GuildUpdate(event) => guild_update::handle(event).await,
            Event::MemberAdd(event) => member_add::handle(event).await,
            Event::MemberRemove(event) => member_remove::handle(event).await,
            Event::MessageCreate(event) => message_create::handle(event).await,
            Event::MessageDelete(event) => message_delete::handle(event).await,
            Event::MessageDeleteBulk(event) => message_delete_bulk::handle(event).await,
            Event::MessageUpdate(event) => message_update::handle(event).await,
            Event::PresenceUpdate(event) => presence_update::handle(event).await,
            Event::ReactionAdd(event) => reaction_add::handle(event).await,
            Event::ReactionRemove(event) => reaction_remove::handle(event).await,
            _ => Ok(()),
        }
    }
}
