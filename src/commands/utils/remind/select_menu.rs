use crate::{commands::utils::remind::set, macros::and_then_or, statics::emojis::ERROR_EMOJI};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let message = input.message.as_ref().unwrap();

    match and_then_or!(
        message.interaction.as_ref(),
        |interaction| Some(input.user.id == interaction.user.id),
        input.guild_id.is_none()
            || input.user.id
                == message
                    .content
                    .chars()
                    .filter(|char| char.is_numeric())
                    .collect::<String>()
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
