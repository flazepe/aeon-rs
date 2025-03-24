use crate::traits::CommandsExt;
use anyhow::Result;
use slashook::chrono::Utc;
use std::fmt::Display;
use twilight_gateway::MessageSender;
use twilight_model::channel::Message;

pub async fn run<T: Display>(message: &Message, _sender: &MessageSender, _args: T) -> Result<()> {
    message.send(format!("Pong! {}s", Utc::now().timestamp_millis() - message.timestamp.as_secs() * 1000)).await
}
