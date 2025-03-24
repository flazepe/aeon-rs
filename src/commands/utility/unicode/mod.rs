mod list;
mod search;

use crate::structs::{command::AeonCommand, command_context::AeonCommandInput};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> =
    LazyLock::new(|| AeonCommand::new("unicode", &[]).subcommand("list", &[], list::run).subcommand("search", &[], search::run));

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
		description = "Does operations with unicode.",
		integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
		subcommands = [
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
			{
				name = "search",
				description = "Searches for unicode characters via query.",
				options = [
					{
						name = "character",
						description = "The character",
						option_type = InteractionOptionType::STRING,
						required = true,
					},
				],
			},
		],
	)]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
