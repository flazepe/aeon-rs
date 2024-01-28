use crate::{
    statics::{FLAZEPE_ID, REST},
    structs::gateway::events::handler::EventHandler,
};
use serde_json::json;
use slashook::structs::channels::Message;
use twilight_gateway::stream::ShardRef;
use twilight_model::gateway::{payload::incoming::MessageCreate, presence::ActivityType, OpCode};

impl<'a> EventHandler {
    pub async fn handle_owner(message: Box<MessageCreate>, shard: ShardRef<'a>) {
        if message.author.id.to_string() != FLAZEPE_ID || !message.content.to_lowercase().starts_with("aeon ") {
            return;
        }

        let prefixless = message.content.chars().skip(5).collect::<String>();
        let (command, args) = prefixless.split_once(' ').unwrap_or(("", ""));

        let mut owner_commands = OwnerCommands { message, shard, args: args.to_string() };

        match command {
            "delete" => owner_commands.delete().await,
            "say" => owner_commands.say().await,
            "status" => owner_commands.status().await,
            _ => {},
        }
    }
}

pub struct OwnerCommands<'a> {
    message: Box<MessageCreate>,
    shard: ShardRef<'a>,
    args: String,
}

impl OwnerCommands<'_> {
    pub async fn delete(&self) {
        let url = self.args.split('/').skip(5).map(|id| id.to_string()).collect::<Vec<String>>().join("/");
        let (channel_id, message_id) = url.split_once('/').unwrap_or(("", ""));
        REST.delete::<()>(format!("channels/{channel_id}/messages/{message_id}")).await.ok();
    }

    pub async fn say(&self) {
        Message::create(&REST, self.message.channel_id.to_string(), &*self.args).await.ok();
    }

    pub async fn status(&mut self) {
        self.shard
            .send(
                json!({
                    "op": OpCode::PresenceUpdate,
                    "d": {
                        "since": null,
                        "activities": [{
                            "name": "yes",
                            "type": ActivityType::Custom,
                            "state": self.args,
                        }],
                        "status": "online",
                        "afk": false,
                    },
                })
                .to_string(),
            )
            .await
            .ok();
    }
}
