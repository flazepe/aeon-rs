mod commands;
mod core;
mod fix_embeds;
mod logs;

use crate::{
    statics::{REDIS, REST},
    structs::gateway::events::fix_embeds::EmbedFixResponse,
};
use serde_json::Value;
use twilight_gateway::{Event, MessageSender};

pub struct EventHandler;

impl EventHandler {
    pub async fn handle(event: Event, sender: MessageSender) {
        let redis = REDIS.get().unwrap();
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
            if redis.get::<Value>(format!("command-responses_{}", message.id)).await.is_ok()
                && let Err(error) = Self::handle_commands(message, &sender).await
            {
                println!("[GATEWAY] An error occurred while handling edited commands: {error:?}");
            }

            if redis.get::<Value>(format!("embed-fix-responses_{}", message.id)).await.is_ok()
                && let Err(error) = Self::handle_fix_embeds(message).await
            {
                println!("[GATEWAY] An error occurred while handling edited fix embeds: {error:?}");
            }
        }

        if let Event::MessageDelete(message) = &event {
            let channel_id = message.channel_id;
            let message_id = message.id;

            if let Ok(command_response) = redis.get::<String>(format!("command-responses_{message_id}")).await {
                _ = REST.delete::<()>(format!("channels/{channel_id}/messages/{command_response}")).await;
            }

            if let Ok(embed_fix_response) = redis.get::<EmbedFixResponse>(format!("embed-fix-responses_{message_id}")).await {
                let embed_fix_response_id = embed_fix_response.id;
                _ = REST.delete::<()>(format!("channels/{channel_id}/messages/{embed_fix_response_id}")).await;
            }
        }

        if let Err(error) = Self::handle_core(&event).await {
            println!("[GATEWAY] An error occurred while handling core event {event_name}: {error:?}");
        }
    }
}
