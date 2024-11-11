use crate::{commands::utility::reminder::set, structs::command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let message = ctx.input.message.as_ref().unwrap();
    let is_reminder_message = message.interaction_metadata.is_none();
    let is_authorized = if is_reminder_message {
        // It's a reminder message (we can tell since the message attached does not have an interaction)
        let is_dm = ctx.input.guild_id.is_none();
        let is_reminder_author = ctx.input.user.id == message.content.chars().filter(|char| char.is_numeric()).collect::<String>();
        is_dm || is_reminder_author
    } else {
        // Else, it's an ephemeral select menu from the message reminder command; we let it pass
        true
    };

    if is_authorized {
        set::run(ctx).await
    } else {
        ctx.respond_error("This isn't your reminder.", true).await
    }
}
