use crate::{
    statics::{emojis::*, google_translate_languages::*},
    structs::api::google_translate::*,
    traits::*,
    *,
};
use anyhow::Context;
use slashook::{command, commands::*, structs::interactions::*};

pub fn get_command() -> Command {
    #[command(
		name = "translate",
		description = "Translate a text to any language.",
		options = [
			{
				name = "text",
				description = "The text to translate",
				option_type = InteractionOptionType::STRING,
				required = true,
			},
			{
				name = "target-language",
				description = "The language to translate the text to",
				option_type = InteractionOptionType::STRING,
				autocomplete = true,
			},
			{
				name = "origin-language",
				description = "The text's origin language",
				option_type = InteractionOptionType::STRING,
				autocomplete = true,
			},
		],
	)]
    async fn translate(input: CommandInput, res: CommandResponder) {
        if input.is_autocomplete() {
            kv_autocomplete!(input, res, GOOGLE_TRANSLATE_LANGUAGES);
        }

        match GoogleTranslate::translate(
            input.get_string_arg("text")?,
            input.get_string_arg("origin-language").unwrap_or("auto".into()),
            input.get_string_arg("target-language").unwrap_or("en".into()),
        )
        .await
        {
            Ok(translation) => {
                res.send_message(translation.format()).await?;
            },
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            },
        }
    }

    translate
}
