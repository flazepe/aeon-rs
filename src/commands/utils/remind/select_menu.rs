use crate::{commands::utils::remind::set, macros::if_else, statics::emojis::ERROR_EMOJI};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let message = input.message.as_ref().unwrap();

    match if_else!(
        // If there is no interaction, we need to verify the user
        message.interaction.is_none(),
        input.guild_id.is_none()
            || input.user.id
                == message
                    .content
                    .chars()
                    .filter(|char| char.is_numeric())
                    .collect::<String>(),
        // Else, it's from the ephemeral select menu
        true
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
    }

    Ok(())
}
