use crate::structs::command_context::{CommandContext, Input};
use anyhow::Result;
use serde_json::json;
use twilight_model::gateway::{OpCode, presence::ActivityType};

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::MessageCommand(_, sender, args) = &ctx.input else { return Ok(()) };

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
