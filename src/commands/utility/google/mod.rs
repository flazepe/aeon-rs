mod assistant;
mod dns;
mod translate;

use crate::structs::{api::google::statics::GOOGLE_DNS_RECORD_TYPES, command::Command};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().subcommand("assistant", assistant::run).subcommand("dns", dns::run).subcommand("translate", translate::run)
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "google",
        description = "Google commands.",
        subcommands = [
			{
                name = "assistant",
                description = "Queries Google Assistant.",
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
    async fn google(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    google
}
