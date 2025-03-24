use crate::{
    statics::{FLAZEPE_ID, REST},
    structs::gateway::events::EventHandler,
};
use anyhow::Result;
use serde_json::{json, to_string};
use slashook::{
    commands::MessageResponse,
    structs::{messages::Message, utils::File},
};
use std::{fmt::Display, process::Command};
use twilight_gateway::MessageSender;
use twilight_model::{
    channel::Message as TwilightMessage,
    gateway::{OpCode, payload::incoming::MessageCreate, presence::ActivityType},
};

impl EventHandler {
    pub async fn handle_owner_commands(event: &MessageCreate, sender: MessageSender) -> Result<()> {
        let message = &event.0;
        let prefix = "";

        if message.author.id.to_string() != FLAZEPE_ID || !message.content.to_lowercase().starts_with(prefix) {
            return Ok(());
        }

        let prefixless = message.content.chars().skip(prefix.len()).collect::<String>();
        let (command, args) = prefixless.split_once(' ').unwrap_or(("", ""));

        let owner_commands = OwnerCommands { message, sender, args: args.to_string() };

        match command {
            "delete" => owner_commands.delete().await,
            "eval" | "evak" => owner_commands.eval().await,
            "status" => owner_commands.status().await,
            _ => Ok(()),
        }
    }
}

struct OwnerCommands<'a> {
    message: &'a TwilightMessage,
    sender: MessageSender,
    args: String,
}

impl OwnerCommands<'_> {
    async fn delete(&self) -> Result<()> {
        let url = self.args.split('/').skip(5).map(|id| id.to_string()).collect::<Vec<String>>().join("/");
        let (channel_id, message_id) = url.split_once('/').unwrap_or(("", ""));
        REST.delete::<()>(format!("channels/{channel_id}/messages/{message_id}")).await?;
        Ok(())
    }

    async fn eval(&self) -> Result<()> {
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
                &generate_eval_context(to_string(&self.message).unwrap_or_else(|_| "{}".into()), code),
                "--input-type",
                if flags.contains('m') { "module" } else { "commonjs" },
            ])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

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
            .await?;
        }

        Ok(())
    }

    async fn status(&self) -> Result<()> {
        self.sender.send(
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
        )?;

        Ok(())
    }
}

fn generate_eval_context<T: Display>(message: T, code: String) -> String {
    format!(
        r#"
            function botFetch(url, options = {{}}) {{
                if (!url.startsWith("http")) url = `https://discord.com/api/${{url}}`;
                if (!options.headers) options.headers = {{}};

                try {{
                    options.headers.authorization = `Bot ${{fs.readFileSync("config.toml", "utf8").split("\n").find(line => line.startsWith("token = ")).split('"')[1]}}`;
                }} catch {{}}

                if (typeof options.body === "string" && !options.headers["content-type"]) options.headers["content-type"] = "application/json";
            
                return fetch(url, options);
            }}

            const message = {message};

            message.reply = (body, filename, extraPayload = {{}}) => {{
                if (filename) {{
                    const formData = new FormData();
                    formData.append("file", new Blob([body]), filename);
                    formData.append("payload_json", JSON.stringify(extraPayload))
                    body = formData;
                }} else {{
                    body = JSON.stringify({{ content: body, ...extraPayload }});
                }}

                return botFetch(`channels/${{message.channel_id}}/messages`, {{ method: "POST", body }});
            }};
            
            {code}
        "#,
    )
}
