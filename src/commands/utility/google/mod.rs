mod dns;
mod translate;

use crate::statics::google::GOOGLE_DNS_RECORD_TYPES;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, InteractionOptionType},
};

pub fn get_command() -> Command {
    #[command(
        name = "google",
        description = "Google commands.",
        subcommands = [
			{
                name = "dns",
                description = "Fetches DNS records of a domain.",
                options = [
					{
						name = "type",
						description = "The record type, such as A, AAAA, MX, NS, PTR, etc.",
						option_type = InteractionOptionType::STRING,
						choices = GOOGLE_DNS_RECORD_TYPES
							.iter()
							.map(|record_type| ApplicationCommandOptionChoice::new(record_type, record_type.to_string()))
							.collect::<Vec<ApplicationCommandOptionChoice>>(),
						required = true,
					},
					{
						name = "domain",
						description = "The domain",
						option_type = InteractionOptionType::STRING,
						required = true,
					},
                ],
            },
			{
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
			}
        ],
    )]
    async fn remind(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref().unwrap_or("") {
            "dns" => dns::run(input, res).await?,
            "translate" => translate::run(input, res).await?,
            _ => {},
        };
    }

    remind
}