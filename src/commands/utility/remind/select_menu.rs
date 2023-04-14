use crate::{commands::utility::remind::set, macros::if_else, statics::emojis::ERROR_EMOJI};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let message = input.message.as_ref().unwrap();

    Ok(
        match if_else!(
            message.interaction.is_none(),
            // If it's a reminder (we can tell since the message attached was not executed from an interaction), we need to verify the user before snoozing
            input.guild_id.is_none() // If it's a DM we don't need to verify (the message content would be empty anyway)
            || input.user.id // Else, parse the ping from the message content
                == message
                    .content
                    .chars()
                    .filter(|char| char.is_numeric())
                    .collect::<String>(),
            // Else, it's an ephemeral select menu from the message reminder command; we let it pass
            true,
        ) {
            true => {
                set::run(input, res).await?;
            },
            false => {
                res.send_message(
                    MessageResponse::from(format!("{ERROR_EMOJI} This isn't your reminder.")).set_ephemeral(true),
                )
                .await?;
            },
        },
    )
}
