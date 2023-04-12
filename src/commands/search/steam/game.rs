use crate::{statics::emojis::ERROR_EMOJI, structs::api::steam::game::SteamGame, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    match SteamGame::get(input.get_string_arg("game")?).await {
        Ok(game) => {
            res.send_message(game.format()).await?;
        },
        Err(error) => {
            res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
        },
    }

    Ok(())
}
