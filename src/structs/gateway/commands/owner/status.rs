use crate::statics::FLAZEPE_ID;
use anyhow::Result;
use serde_json::json;
use std::fmt::Display;
use twilight_gateway::MessageSender;
use twilight_model::gateway::{OpCode, payload::incoming::MessageCreate, presence::ActivityType};

pub async fn run<T: Display>(event: &MessageCreate, sender: &MessageSender, args: T) -> Result<()> {
    if event.author.id.to_string() != FLAZEPE_ID {
        return Ok(());
    }

    sender.send(
        json!({
            "op": OpCode::PresenceUpdate,
            "d": {
                "since": null,
                "activities": [{
                    "name": "yes",
                    "type": ActivityType::Custom,
                    "state": args.to_string(),
                }],
                "status": "online",
                "afk": false,
            },
        })
        .to_string(),
    )?;

    Ok(())
}
