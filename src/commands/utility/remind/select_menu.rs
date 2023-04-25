use crate::{commands::utility::remind::set, structs::interaction::Interaction};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };
    let message = input.message.as_ref().unwrap();

    if match message.interaction.is_none() {
        // If it's a reminder (we can tell since the message attached does not have an interaction), we need to verify the user before snoozing
        true => {
            input.guild_id.is_none() // If it's a DM we don't need to verify (the message content would be empty anyway)
            || input.user.id // Else, parse the ping from the message content
                == message
                    .content
                    .chars()
                    .filter(|char| char.is_numeric())
                    .collect::<String>()
        },
        // Else, it's an ephemeral select menu from the message reminder command; we let it pass
        false => true,
    } {
        set::run(input, res).await
    } else {
        interaction.respond_error("This isn't your reminder.", true).await
    }
}
