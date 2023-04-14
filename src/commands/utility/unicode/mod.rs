mod list;
mod search;

use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

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
        match input.subcommand.as_deref().unwrap_or("") {
            "list" => list::run(input, res).await?,
            "search" => search::run(input, res).await?,
            _ => {},
        };
    }

    unicode
}
