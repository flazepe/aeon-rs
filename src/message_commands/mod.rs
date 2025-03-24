use anyhow::Result;
use std::fmt::Display;
use twilight_gateway::MessageSender;
use twilight_model::channel::Message;

mod owner;
mod utility;

pub async fn run<T: Display, U: Display>(message: &Message, sender: &MessageSender, command: T, args: U) -> Result<()> {
    match command.to_string().as_str() {
        // Owner
        "delete" => owner::delete::run(message, sender, args).await,
        "eval" | "evak" => owner::eval::run(message, sender, args).await,
        "status" => owner::status::run(message, sender, args).await,

        // Utility
        "convert" | "cc" => utility::convert_currency::run(message, sender, args).await,
        "ping" => utility::ping::run(message, sender, args).await,

        _ => Ok(()),
    }
}
