mod list;
mod search;

use crate::structs::command::Command;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| Command::new().subcommand("list", list::run).subcommand("search", search::run));

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "unicode",
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
				description = "Searches for a unicode character via query.",
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
    async fn unicode(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    unicode
}
