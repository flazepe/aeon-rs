use crate::{commands::utility::reminder::set, structs::command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let message = ctx.input.message.as_ref().unwrap();
    let is_reminder_message = message.interaction_metadata.is_none();
    let is_authorized = match is_reminder_message {
        // It's a reminder message (we can tell since the message attached does not have an interaction)
        true => {
            let is_dm = ctx.input.guild_id.is_none();
            let is_reminder_author = ctx.input.user.id == message.content.chars().filter(|char| char.is_numeric()).collect::<String>();
            is_dm || is_reminder_author
        },
        // Else, it's an ephemeral select menu from the message reminder command; we let it pass
        false => true,
    };

    match is_authorized {
        true => set::run(ctx).await,
        false => ctx.respond_error("This isn't your reminder.", true).await,
    }
}
