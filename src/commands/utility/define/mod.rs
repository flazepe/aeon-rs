mod dictionary;
mod urban_dictionary;

use crate::structs::command::Command;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: Lazy<Command> =
    Lazy::new(|| Command::new().subcommand("dictionary", dictionary::run).subcommand("urban-dictionary", urban_dictionary::run));

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "define",
		description = "Defines a word.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        subcommands = [
			{
				name = "dictionary",
				description = "Defines a word from the dictionary.",
				options = [
					{
						name = "word",
						description = "The word",
						option_type = InteractionOptionType::STRING,
						required = true,
					},
					{
						name = "show",
						description = "Whether to publicly show the response",
						option_type = InteractionOptionType::BOOLEAN,
					},
				],
			},
            {
				name = "urban-dictionary",
				description = "Defines a word from the Urban Dictionary.",
				options = [
					{
						name = "word",
						description = "The word",
						option_type = InteractionOptionType::STRING,
						required = true,
					},
					{
						name = "show",
						description = "Whether to publicly show the response",
						option_type = InteractionOptionType::BOOLEAN,
					},
				],
			}
        ],
    )]
    async fn define(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    define
}
