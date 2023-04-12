use crate::{functions::if_else_option, statics::emojis::ERROR_EMOJI, structs::snipes::Snipes, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    match Snipes::new(
        if_else_option(
            input.get_channel_arg("channel").ok(),
            |channel| &channel.id,
            input.channel_id.as_ref().unwrap(),
        ),
        input.get_bool_arg("edit")?,
        input.get_bool_arg("list")?,
    )
    .to_response()
    {
        Ok(response) => {
            res.send_message(response).await?;
        },
        Err(error) => {
            res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
        },
    }

    Ok(())
}
