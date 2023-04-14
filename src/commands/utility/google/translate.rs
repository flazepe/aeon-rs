use crate::{
    functions::hashmap_autocomplete,
    statics::{emojis::ERROR_EMOJI, google::GOOGLE_TRANSLATE_LANGUAGES},
    structs::api::google::Google,
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    if input.is_autocomplete() {
        return Ok(hashmap_autocomplete(input, res, GOOGLE_TRANSLATE_LANGUAGES.iter()).await?);
    }

    match Google::translate(
        input.get_string_arg("text")?,
        input.get_string_arg("origin-language").unwrap_or("auto".into()),
        input.get_string_arg("target-language").unwrap_or("en".into()),
    )
    .await
    {
        Ok(translation) => res.send_message(translation.format()).await?,
        Err(error) => res.send_message(format!("{ERROR_EMOJI} {error}")).await?,
    };

    Ok(())
}
