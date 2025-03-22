mod fix_embeds;
mod owner_commands;

use twilight_gateway::{Event, MessageSender};

pub struct EventHandler;

impl EventHandler {
    pub async fn handle(event: Event, sender: MessageSender) {
        let event_name = format!("{:?}", event.kind());

        if let Event::MessageCreate(message) = &event {
            if let Err(error) = Self::handle_owner_commands(message, sender).await {
                println!("[GATEWAY] An error occurred while handling owner commands: {error:?}");
            }

            if let Err(error) = Self::handle_fix_embeds(message).await {
                println!("[GATEWAY] An error occurred while handling embed fix: {error:?}");
            }
        }

        let result = match event {
            Event::GuildDelete(guild) => Self::on_guild_delete(guild).await,
            Event::MessageCreate(message) => Self::on_message_create(message).await,
            Event::MessageDelete(message) => Self::on_message_delete(message).await,
            Event::MessageDeleteBulk(data) => Self::on_message_delete_bulk(data).await,
            Event::MessageUpdate(message) => Self::on_message_update(message).await,
            Event::PresenceUpdate(presence) => Self::on_presence_update(presence).await,
            Event::ReactionAdd(reaction) => Self::on_reaction_add(reaction).await,
            Event::ReactionRemove(reaction) => Self::on_reaction_remove(reaction).await,
            _ => Ok(()),
        };

        if let Err(error) = result {
            println!("[GATEWAY] An error occurred while handling event {event_name}: {error:?}");
        }
    }
}
