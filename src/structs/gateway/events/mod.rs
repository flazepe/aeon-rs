mod commands;
mod core;
mod fix_embeds;
mod logs;

use crate::statics::{CACHE, REST};
use twilight_gateway::{Event, MessageSender};

pub struct EventHandler;

impl EventHandler {
    pub async fn handle(event: Event, sender: MessageSender) {
        let event_name = format!("{:?}", event.kind());

        if let Err(error) = Self::handle_logs(&event).await {
            println!("[GATEWAY] An error occurred while handling log event {event_name}: {error:?}");
        }

        if let Event::MessageCreate(message) = &event {
            if let Err(error) = Self::handle_commands(message, &sender).await {
                println!("[GATEWAY] An error occurred while handling commands: {error:?}");
            }

            if let Err(error) = Self::handle_fix_embeds(message).await {
                println!("[GATEWAY] An error occurred while handling embed fix: {error:?}");
            }
        }

        if let Event::MessageUpdate(message) = &event {
            if let Err(error) = Self::handle_commands(message, &sender).await {
                println!("[GATEWAY] An error occurred while handling commands: {error:?}");
            }
        }

        if let Event::MessageDelete(message) = &event {
            let message_id = message.id.to_string();
            let command_response = CACHE.command_responses.read().unwrap().get(&message_id).cloned();
            CACHE.command_responses.write().unwrap().remove(&message_id);

            if let Some(command_response) = command_response {
                let _ = command_response.delete(&REST).await;
            }
        }

        if let Err(error) = Self::handle_core(&event).await {
            println!("[GATEWAY] An error occurred while handling core event {event_name}: {error:?}");
        }
    }
}
