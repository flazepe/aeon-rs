mod list;
mod search;

use crate::structs::command::AeonCommand;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| AeonCommand::new().subcommand("list", list::run).subcommand("search", search::run));

pub fn get_command() -> Command {
    #[command(
		name = "unicode",
		description = "Does operations with unicode.",
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
		],
	)]
    async fn unicode(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    unicode
}
