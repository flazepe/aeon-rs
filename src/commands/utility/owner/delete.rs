use crate::{
    statics::REST,
    structs::command_context::{CommandContext, Input},
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::MessageCommand { message: _, sender: _, args } = &ctx.input else { return Ok(()) };

    let url = args.split('/').skip(5).map(|id| id.to_string()).collect::<Vec<String>>().join("/");
    let (channel_id, message_id) = url.split_once('/').unwrap_or(("", ""));
    let _ = REST.delete::<()>(format!("channels/{channel_id}/messages/{message_id}")).await;

    Ok(())
}
