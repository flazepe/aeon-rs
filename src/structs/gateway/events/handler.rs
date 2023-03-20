use twilight_gateway::{stream::ShardRef, Event};

static EXCLUDED_EVENTS: [&str; 2] = ["GatewayHeartbeatAck", "GuildCreate"];

pub struct EventHandler {}

impl EventHandler {
    pub fn handle(shard: ShardRef, event: Event) {
        let event_name = format!("{:?}", event.kind());

        if !EXCLUDED_EVENTS.contains(&event_name.as_str()) {
            println!("[EVENT] [SHARD {}] {event_name}", shard.id());
        }

        match event {
            Event::MessageCreate(message) => EventHandler::on_message_create(message),
            Event::MessageDelete(message) => EventHandler::on_message_delete(message),
            Event::MessageUpdate(message) => EventHandler::on_message_update(message),
            _ => {}
        }
    }
}
