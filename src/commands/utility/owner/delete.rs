use crate::{
    statics::REST,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::MessageCommand(_, args, _) = &ctx.command_input else { return Ok(()) };

    let url = args.split('/').skip(5).map(|id| id.to_string()).collect::<Vec<String>>().join("/");
    let (channel_id, message_id) = url.split_once('/').unwrap_or(("", ""));
    let _ = REST.delete::<()>(format!("channels/{channel_id}/messages/{message_id}")).await;

    Ok(())
}
