use crate::{structs::api::xe::Xe, traits::CommandsExt};
use anyhow::Result;
use std::fmt::Display;
use twilight_gateway::MessageSender;
use twilight_model::channel::Message;

pub async fn run<T: Display>(message: &Message, _sender: &MessageSender, args: T) -> Result<()> {
    let args = args.to_string();
    let mut split = args.split(' ');

    let (Some(amount), Some(origin_currency), Some(target_currency)) = (split.next(), split.next(), split.next()) else {
        return message.send_error("Invalid arguments.").await;
    };

    let Ok(amount) = amount.parse::<f64>() else {
        return message.send_error("Invalid amount.").await;
    };

    match Xe::convert(amount, origin_currency, target_currency).await {
        Ok(xe_conversion) => message.send_success(xe_conversion.format()).await,
        Err(error) => message.send_error(error).await,
    }
}
