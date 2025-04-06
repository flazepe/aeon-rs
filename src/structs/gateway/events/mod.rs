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
            if CACHE.command_responses.read().unwrap().contains_key(&message.id.to_string()) {
                if let Err(error) = Self::handle_commands(message, &sender).await {
                    println!("[GATEWAY] An error occurred while handling commands: {error:?}");
                }
            }
        }

        if let Event::MessageDelete(message) = &event {
            let command_response = CACHE.command_responses.write().unwrap().remove(&message.id.to_string());

            if let Some(command_response) = command_response {
                _ = command_response.delete(&REST).await;
            }
        }

        if let Err(error) = Self::handle_core(&event).await {
            println!("[GATEWAY] An error occurred while handling core event {event_name}: {error:?}");
        }
    }
}
