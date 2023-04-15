use crate::{statics::emojis::ERROR_EMOJI, structs::snipes::Snipes, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    match Snipes::new(
        input
            .get_channel_arg("channel")
            .map_or(input.channel_id.as_ref().unwrap(), |channel| &channel.id),
        input.get_bool_arg("edit")?,
        input.get_bool_arg("list")?,
    )
    .to_response()
    {
        Ok(response) => res.send_message(response).await?,
        Err(error) => res.send_message(format!("{ERROR_EMOJI} {error}")).await?,
    };

    Ok(())
}
