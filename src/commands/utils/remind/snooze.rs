use crate::{commands::utils::remind::set, statics::emojis::ERROR_EMOJI};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let message = input.message.as_ref().unwrap();

    match input.guild_id.is_none()
        || input.user.id
            == message
                .content
                .chars()
                .skip(2)
                .take(message.content.len() - 3)
                .collect::<String>()
    {
        true => {
            set::run(input, res).await?;
        },
        false => {
            res.defer(true).await?;

            res.send_message(format!("{ERROR_EMOJI} This isn't your reminder."))
                .await?;
        },
    }

    Ok(())
}
