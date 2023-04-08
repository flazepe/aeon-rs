use crate::{statics::emojis::ERROR_EMOJI, structs::api::vndb::Vndb, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    match Vndb::new()
        .search_character(input.get_string_arg("character")?)
        .await
    {
        Ok(mut character) => {
            res.send_message(character.remove(0).format()).await?;
        },
        Err(error) => {
            res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
        },
    }

    Ok(())
}
