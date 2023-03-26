use crate::{statics::emojis::ERROR_EMOJI, structs::snipes::ReactionSnipes, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let message = input.get_string_arg("message")?;

    match ReactionSnipes::new(input.guild_id.as_ref().unwrap(), message.split("/").last().unwrap()).to_response() {
        Ok(response) => {
            res.send_message(response).await?;
        },
        Err(error) => {
            res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
        },
    }

    Ok(())
}
