use crate::{commands::utility::reminder::set, structs::command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let message = ctx.input.message.as_ref().unwrap();

    if match message.interaction.is_none() {
        // If it's a reminder (we can tell since the message attached does not have an interaction), we need to verify the user before snoozing
        true => {
            ctx.input.guild_id.is_none() // If it's a DM we don't need to verify (the message content would be empty anyway)
            ||  ctx.input.user.id // Else, parse the ping from the message content
                == message
                    .content
                    .chars()
                    .filter(|char| char.is_numeric())
                    .collect::<String>()
        },
        // Else, it's an ephemeral select menu from the message reminder command; we let it pass
        false => true,
    } {
        set::run(ctx).await
    } else {
        ctx.respond_error("This isn't your reminder.", true).await
    }
}
