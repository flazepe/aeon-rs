use crate::{
    functions::hashmap_autocomplete,
    statics::{emojis::ERROR_EMOJI, google_translate_languages::GOOGLE_TRANSLATE_LANGUAGES},
    structs::api::google_translate::GoogleTranslate,
    traits::ArgGetters,
};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

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
            return hashmap_autocomplete(input, res, GOOGLE_TRANSLATE_LANGUAGES.iter()).await?;
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
