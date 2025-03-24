use anyhow::Result;
use std::fmt::Display;
use twilight_gateway::MessageSender;
use twilight_model::gateway::payload::incoming::MessageCreate;

mod owner;
mod utility;

pub async fn run<T: Display, U: Display>(event: &MessageCreate, sender: &MessageSender, command: T, args: U) -> Result<()> {
    match command.to_string().as_str() {
        // Owner
        "delete" => owner::delete::run(event, sender, args).await,
        "eval" => owner::eval::run(event, sender, args).await,
        "status" => owner::status::run(event, sender, args).await,

        // Utility
        "ping" => utility::ping::run(event, sender, args).await,

        _ => Ok(()),
    }
}
