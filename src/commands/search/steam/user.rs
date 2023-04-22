use crate::{statics::emojis::ERROR_EMOJI, structs::api::steam::Steam, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    Ok(match Steam::get_user(input.get_string_arg("user")?).await {
        Ok(user) => {
            res.send_message(user.format()).await?;
        },
        Err(error) => {
            res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} {error}")).set_ephemeral(true))
                .await?;
        },
    })
}
