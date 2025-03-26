use crate::{
    commands::utility::reminder::set,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::{Result, bail};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
    let message = input.message.as_ref().unwrap();
    let is_reminder_message = message.interaction_metadata.is_none();
    let is_authorized = if is_reminder_message {
        // It's a reminder message (we can tell since the message attached does not have an interaction)
        let is_dm = input.guild_id.is_none();
        let is_reminder_author = input.user.id == message.content.chars().filter(|char| char.is_numeric()).collect::<String>();
        is_dm || is_reminder_author
    } else {
        // Else, it's an ephemeral select menu from the message reminder command; we let it pass
        true
    };

    if !is_authorized {
        bail!("This isn't your reminder.");
    }

    set::run(ctx).await
}
