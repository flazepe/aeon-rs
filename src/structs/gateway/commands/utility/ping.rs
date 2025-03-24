use crate::statics::REST;
use anyhow::Result;
use slashook::structs::messages::Message;
use std::fmt::Display;
use twilight_gateway::MessageSender;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn run<T: Display>(event: &MessageCreate, _sender: &MessageSender, _args: T) -> Result<()> {
    let _ = Message::create(&REST, event.channel_id, "Pong!").await;
    Ok(())
}
