use twilight_gateway::{stream::ShardRef, Event};

pub struct EventHandler {}

impl EventHandler {
    pub fn handle(event: Event, _shard: ShardRef) {
        match event {
            Event::MessageCreate(message) => EventHandler::on_message_create(message),
            Event::MessageDelete(message) => EventHandler::on_message_delete(message),
            Event::MessageDeleteBulk(data) => EventHandler::on_message_delete_bulk(data),
            Event::MessageUpdate(message) => EventHandler::on_message_update(message),
            Event::ReactionRemove(reaction) => EventHandler::on_reaction_remove(reaction),
            _ => {},
        }
    }
}
