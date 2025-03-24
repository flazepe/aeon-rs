use crate::{structs::api::google::Google, traits::CommandsExt};
use anyhow::Result;
use std::fmt::Display;
use twilight_gateway::MessageSender;
use twilight_model::channel::Message;

pub async fn run<T: Display>(message: &Message, _sender: &MessageSender, args: T) -> Result<()> {
    let args = args.to_string();
    let mut split = args.split(' ');

    let Some(target_language) = split.next() else {
        return message.send_error("Please provide the target language.").await;
    };

    let text = split.collect::<Vec<&str>>().join(" ");

    if text.is_empty() {
        return message.send_error("Please provide a text to translate.").await;
    };

    match Google::translate(text, "auto", target_language).await {
        Ok(xe_conversion) => message.send(xe_conversion.format()).await,
        Err(error) => message.send_error(error).await,
    }
}
