use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput},
    database::{Database, redis::keys::RedisKey},
};
use anyhow::Result;
use serde_json::json;
use std::sync::Arc;
use twilight_model::gateway::{OpCode, presence::ActivityType};

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::MessageCommand(_, args, sender) = &ctx.command_input else { return Ok(()) };

    Database::get_redis()?.set(&RedisKey::CustomStatus, args.get_content(), None).await?;

    sender.send(
        json!({
            "op": OpCode::PresenceUpdate,
            "d": {
                "since": null,
                "activities": [{
                    "name": "yes",
                    "type": ActivityType::Custom,
                    "state": args.get_content(),
                }],
                "status": "online",
                "afk": false,
            },
        })
        .to_string(),
    )?;

    Ok(())
}
