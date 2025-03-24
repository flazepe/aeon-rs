mod commands;
mod core;
mod fix_embeds;
mod logs;

use twilight_gateway::{Event, MessageSender};

pub struct EventHandler;

impl EventHandler {
    pub async fn handle(event: Event, sender: MessageSender) {
        let event_name = format!("{:?}", event.kind());

        if let Err(error) = Self::handle_logs(&event).await {
            println!("[GATEWAY] An error occurred while handling log event {event_name}: {error:?}");
        }

        if let Event::MessageCreate(event) = &event {
            if let Err(error) = Self::handle_commands(event, &sender).await {
                println!("[GATEWAY] An error occurred while handling commands: {error:?}");
            }

            if let Err(error) = Self::handle_fix_embeds(event).await {
                println!("[GATEWAY] An error occurred while handling embed fix: {error:?}");
            }
        }

        if let Err(error) = Self::handle_core(&event).await {
            println!("[GATEWAY] An error occurred while handling core event {event_name}: {error:?}");
        }
    }
}
