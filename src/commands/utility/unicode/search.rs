use crate::{statics::emojis::ERROR_EMOJI, structs::unicode::UnicodeCharacter, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    match UnicodeCharacter::get(input.get_string_arg("query")?).await {
        Ok(unicode_character) => res.send_message(unicode_character.format()).await?,
        Err(error) => {
            res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} {error}")).set_ephemeral(true))
                .await?
        },
    };

    Ok(())
}
