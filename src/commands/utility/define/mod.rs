mod dictionary;
mod urban_dictionary;

use crate::structs::{command::AeonCommand, command_context::AeonCommandInput};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("define", &["d"]).subcommand("dictionary", &["dict", "word"], dictionary::run).subcommand(
        "urban-dictionary",
        &["ud", "urban"],
        urban_dictionary::run,
    )
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
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
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
