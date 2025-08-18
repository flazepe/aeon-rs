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
                println!("[GATEWAY] An error occurred while handling fix embeds: {error:?}");
            }
        }

        if let Event::MessageUpdate(message) = &event {
            if CACHE.discord.command_responses.read().unwrap().contains_key(&message.id.to_string())
                && let Err(error) = Self::handle_commands(message, &sender).await
            {
                println!("[GATEWAY] An error occurred while handling edited commands: {error:?}");
            }

            if CACHE.discord.embed_fix_responses.read().unwrap().contains_key(&message.id.to_string())
                && let Err(error) = Self::handle_fix_embeds(message).await
            {
                println!("[GATEWAY] An error occurred while handling edited fix embeds: {error:?}");
            }
        }

        if let Event::MessageDelete(message) = &event {
            let message_id = message.id.to_string();
            let command_response = CACHE.discord.command_responses.read().unwrap().get(&message.id.to_string()).cloned();

            if let Some(command_response) = command_response {
                _ = command_response.delete(&REST).await;
            }

            CACHE
                .discord
                .command_responses
                .write()
                .unwrap()
                .retain(|id, command_response| id != &message_id && command_response.id.as_deref().unwrap_or_default() != message_id);

            let embed_fix_response = CACHE.discord.embed_fix_responses.read().unwrap().get(&message.id.to_string()).cloned();

            if let Some(embed_fix_response) = embed_fix_response {
                _ = embed_fix_response.delete(&REST).await;
            }

            CACHE
                .discord
                .embed_fix_responses
                .write()
                .unwrap()
                .retain(|id, embed_fix_response| id != &message_id && embed_fix_response.id.as_deref().unwrap_or_default() != message_id);
        }

        if let Err(error) = Self::handle_core(&event).await {
            println!("[GATEWAY] An error occurred while handling core event {event_name}: {error:?}");
        }
    }
}
