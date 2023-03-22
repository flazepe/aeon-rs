use crate::{constants::*, structs::unicode::*, traits::*};
use slashook::{command, commands::*, structs::interactions::*};

pub fn get_command() -> Command {
    #[command(
		name = "unicode",
		description = "Does operations with unicode.",
		subcommands = [
			{
				name = "search",
				description = "Searches for a unicode emoji via query.",
				options = [
					{
						name = "query",
						description = "The query",
						option_type = InteractionOptionType::STRING,
						required = true,
					},
				],
			},
			{
				name = "list",
				description = "Lists unicodes from a text.",
				options = [
					{
						name = "text",
						description = "The text",
						option_type = InteractionOptionType::STRING,
						required = true,
					},
				],
			},
		]
	)]
    async fn unicode(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref().unwrap_or("") {
            "search" => match UnicodeCharacter::get(input.get_string_arg("query")?).await {
                Ok(unicode_character) => {
                    res.send_message(unicode_character.format()).await?;
                }
                Err(error) => {
                    res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                }
            },
            "list" => {
                res.send_message(UnicodeCharacters::get(input.get_string_arg("text")?).format())
                    .await?
            }
            _ => {}
        }
    }

    unicode
}
