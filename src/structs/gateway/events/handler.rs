use twilight_gateway::{stream::ShardRef, Event};

pub struct EventHandler;

impl<'a> EventHandler {
    pub async fn handle(event: Event, shard: ShardRef<'a>) {
        match event {
            Event::MessageCreate(message) => {
                Self::handle_owner(message.clone(), shard).await;
                Self::on_message_create(message).await;
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
