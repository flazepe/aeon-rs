use crate::{commands::utility::remind::set, macros::if_else, structs::interaction::Interaction};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };
    let message = input.message.as_ref().unwrap();

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
        true => set::run(input, res).await,
        false => interaction.respond_error("This isn't your reminder.", true).await,
    }
}
