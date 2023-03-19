use crate::structs::gateway::events::{message_create, message_delete, message_update};
use twilight_gateway::{stream::ShardRef, Event};

pub fn handle(_shard: ShardRef, event: Event) {
    match event {
        Event::MessageCreate(message) => message_create::handle(message),
        Event::MessageDelete(message) => message_delete::handle(message),
        Event::MessageUpdate(message) => message_update::handle(message),
        _ => {}
    }
}
