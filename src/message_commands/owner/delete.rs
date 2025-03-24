use crate::statics::{FLAZEPE_ID, REST};
use anyhow::Result;
use std::fmt::Display;
use twilight_gateway::MessageSender;
use twilight_model::channel::Message;

pub async fn run<T: Display>(message: &Message, _sender: &MessageSender, args: T) -> Result<()> {
    if message.author.id.to_string() != FLAZEPE_ID {
        return Ok(());
    }

    let url = args.to_string().split('/').skip(5).map(|id| id.to_string()).collect::<Vec<String>>().join("/");
    let (channel_id, message_id) = url.split_once('/').unwrap_or(("", ""));
    let _ = REST.delete::<()>(format!("channels/{channel_id}/messages/{message_id}")).await;

    Ok(())
}
