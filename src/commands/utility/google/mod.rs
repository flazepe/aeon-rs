mod assistant;
mod dns;
mod translate;

use crate::structs::{api::google::statics::GOOGLE_DNS_RECORD_TYPES, command::AeonCommand, command_context::AeonCommandInput};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("google", &["g", "goog"])
        .subcommand("assistant", &["ass"], assistant::run)
        .subcommand("dns", &[], dns::run)
        .subcommand("translate", &["tl", "tr", "trans"], translate::run)
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Google commands.",
		integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
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
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
