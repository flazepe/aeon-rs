mod commands;
mod core;
mod fix_embeds;
mod logs;

use crate::{
    statics::REST,
    structs::{
        database::{Database, redis::keys::RedisKey},
        gateway::events::fix_embeds::EmbedFixResponse,
    },
};
use serde_json::Value;
use tracing::error;
use twilight_gateway::{Event, MessageSender};

pub struct EventHandler;

impl EventHandler {
    pub async fn handle(event: Event, sender: MessageSender) {
        let event_name = format!("{:?}", event.kind());

        if let Err(error) = Self::handle_logs(&event).await {
            error!(target: "Gateway", "An error occurred while handling log event {event_name}: {error:#?}");
        }

        if let Event::MessageCreate(message) = &event {
            if let Err(error) = Self::handle_commands(message, &sender).await {
                error!(target: "Gateway", "An error occurred while handling commands: {error:#?}");
            }

            if let Err(error) = Self::handle_fix_embeds(message).await {
                error!(target: "Gateway", "An error occurred while handling fix embeds: {error:#?}");
            }
        }

        if let Event::MessageUpdate(message) = &event {
            let Some(guild_id) = message.guild_id else { return };
            let channel_id = message.channel_id;
            let message_id = message.id;

            let Ok(redis) = Database::get_redis() else { return };
            let key = RedisKey::GuildChannelMessageCommandResponse(guild_id.to_string(), channel_id.to_string(), message_id.to_string());
            let has_response = redis.get::<Value>(&key).await.is_ok();

            if has_response && let Err(error) = Self::handle_commands(message, &sender).await {
                error!(target: "Gateway", "An error occurred while handling edited commands: {error:#?}");
            }

            let key = RedisKey::GuildChannelMessageEmbedFixResponse(guild_id.to_string(), channel_id.to_string(), message_id.to_string());
            let has_response = redis.get::<Value>(&key).await.is_ok();

            if has_response && let Err(error) = Self::handle_fix_embeds(message).await {
                error!(target: "Gateway", "An error occurred while handling edited fix embeds: {error:#?}");
            }
        }

        if let Event::MessageDelete(message) = &event {
            let Some(guild_id) = message.guild_id else { return };
            let channel_id = message.channel_id;
            let message_id = message.id;

            let Ok(redis) = Database::get_redis() else { return };
            let key = RedisKey::GuildChannelMessageCommandResponse(guild_id.to_string(), channel_id.to_string(), message_id.to_string());

            if let Ok(command_response) = redis.get::<String>(&key).await {
                _ = REST.delete::<()>(format!("channels/{channel_id}/messages/{command_response}")).await;
            }

            let key = RedisKey::GuildChannelMessageEmbedFixResponse(guild_id.to_string(), channel_id.to_string(), message_id.to_string());

            if let Ok(embed_fix_response) = redis.get::<EmbedFixResponse>(&key).await {
                let embed_fix_response_id = embed_fix_response.id;
                _ = REST.delete::<()>(format!("channels/{channel_id}/messages/{embed_fix_response_id}")).await;
            }
        }

        if let Err(error) = Self::handle_core(&event).await {
            error!(target: "Gateway", "An error occurred while handling core event {event_name}: {error:#?}");
        }
    }
}
