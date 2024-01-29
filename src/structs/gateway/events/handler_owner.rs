use crate::{
    statics::{FLAZEPE_ID, REST},
    structs::gateway::events::handler::EventHandler,
};
use serde_json::json;
use slashook::{
    commands::MessageResponse,
    structs::{channels::Message, utils::File},
};
use std::process::Command;
use twilight_gateway::stream::ShardRef;
use twilight_model::gateway::{payload::incoming::MessageCreate, presence::ActivityType, OpCode};

impl<'a> EventHandler {
    pub async fn handle_owner(message: Box<MessageCreate>, shard: ShardRef<'a>) {
        let prefix = "";

        if message.author.id.to_string() != FLAZEPE_ID || !message.content.to_lowercase().starts_with(prefix) {
            return;
        }

        let prefixless = message.content.chars().skip(prefix.len()).collect::<String>();
        let (command, args) = prefixless.split_once(' ').unwrap_or(("", ""));

        let mut owner_commands = OwnerCommands { message, shard, args: args.to_string() };

        match command {
            "delete" => owner_commands.delete().await,
            "eval" | "evak" => owner_commands.eval().await,
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

    pub async fn eval(&self) {
        let mut code = self.args.clone();
        let mut flags = code.split(' ').last().unwrap_or("").to_string();

        if flags.starts_with('-') && flags.chars().skip(1).all(|char| char.is_alphabetic()) {
            code = code.trim_end_matches(&flags).trim_end().to_string();
        } else {
            flags.clear();
        }

        // Codeblock trim
        if code.starts_with("```") {
            code = code.trim_end_matches("```").chars().skip(if code.starts_with("```js") { 5 } else { 3 }).collect::<String>();
        }

        let mut text = "No result.".to_string();

        if let Ok(output) = Command::new("node")
            .args([
                if flags.contains('m') { "-e" } else { "-p" },
                &code,
                "--input-type",
                if flags.contains('m') { "module" } else { "commonjs" },
            ])
            .output()
        {
            let stdout = String::from_utf8(output.stdout).unwrap_or("".into());
            let stderr = String::from_utf8(output.stderr).unwrap_or("".into());

            text = format!("{}", output.status);

            if !stdout.trim().is_empty() {
                text += &format!("\nstdout:\n```js\n{}```", stdout.replace('`', "\u{200b}`"));
            }

            if !stderr.trim().is_empty() {
                text += &format!("\nstderr:\n```js\n{}```", stderr.replace('`', "\u{200b}`"));
            }
        }

        if !flags.contains('s') {
            Message::create(
                &REST,
                self.message.channel_id,
                if text.len() > 2000 { MessageResponse::from(File::new("result.txt", text)) } else { MessageResponse::from(text) },
            )
            .await
            .ok();
        }
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
