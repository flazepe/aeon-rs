use twilight_gateway::{Event, MessageSender};

pub struct EventHandler;

impl EventHandler {
    pub async fn handle(event: Event, sender: MessageSender) {
        match event {
            Event::GuildDelete(guild) => Self::on_guild_delete(guild).await,
            Event::MessageCreate(message) => {
                Self::handle_owner(message.clone(), sender).await;

                if let Err(error) = Self::on_message_create(message).await {
                    println!("[MESSAGE_CREATE] {error:?}");
                };
            },
            Event::MessageDelete(message) => Self::on_message_delete(message).await,
            Event::MessageDeleteBulk(data) => Self::on_message_delete_bulk(data).await,
            Event::MessageUpdate(message) => Self::on_message_update(message).await,
            Event::PresenceUpdate(presence) => Self::on_presence_update(presence).await,
            Event::ReactionAdd(reaction) => Self::on_reaction_add(reaction).await,
            Event::ReactionRemove(reaction) => Self::on_reaction_remove(reaction).await,
            _ => {},
        }
    }
}
