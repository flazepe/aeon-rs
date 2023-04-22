use crate::{statics::emojis::ERROR_EMOJI, structs::snipes::Snipes, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    match Snipes::new(
        input
            .get_channel_arg("channel")
            .map_or(input.channel_id.as_ref().unwrap(), |channel| &channel.id),
        input.get_bool_arg("edit").unwrap_or(false),
        input.get_bool_arg("list").unwrap_or(false),
    )
    .to_response()
    {
        Ok(response) => res.send_message(response).await?,
        Err(error) => {
            res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} {error}")).set_ephemeral(true))
                .await?
        },
    };

    Ok(())
}
