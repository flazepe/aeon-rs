use crate::{statics::emojis::ERROR_EMOJI, structs::api::steam::user::SteamUser, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    match SteamUser::get(input.get_string_arg("user")?).await {
        Ok(user) => {
            res.send_message(user.format()).await?;
        },
        Err(error) => {
            res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
        },
    }

    Ok(())
}
