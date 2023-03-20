use twilight_gateway::{stream::ShardRef, Event};

pub struct EventHandler {}

impl EventHandler {
    pub fn handle(shard: ShardRef, event: Event) {
        println!("[EVENT] [SHARD {}] {:?}", shard.id(), event.kind());

        match event {
            Event::MessageCreate(message) => EventHandler::on_message_create(message),
            Event::MessageDelete(message) => EventHandler::on_message_delete(message),
            Event::MessageUpdate(message) => EventHandler::on_message_update(message),
            _ => {}
        }
    }
}
