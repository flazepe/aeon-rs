use twilight_gateway::{stream::ShardRef, Event};

pub struct EventHandler {}

impl EventHandler {
    pub fn handle(event: Event, _shard: ShardRef) {
        match event {
            Event::MessageCreate(message) => Self::on_message_create(message),
            Event::MessageDelete(message) => Self::on_message_delete(message),
            Event::MessageDeleteBulk(data) => Self::on_message_delete_bulk(data),
            Event::MessageUpdate(message) => Self::on_message_update(message),
            Event::ReactionRemove(reaction) => Self::on_reaction_remove(reaction),
            _ => {},
        }
    }
}
