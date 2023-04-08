mod character;
mod visual_novel;

use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
        name = "vndb",
        description = "Searches for various resources from Visual Novel Database.",
        subcommands = [
			{
                name = "visual-novel",
                description = "Fetches a visual novel from Visual Novel Database.",
                options = [
                    {
                        name = "visual-novel",
                        description = "The visual novel",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
					{
                        name = "search",
                        description = "Whether to search",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
                ],
            },
            {
                name = "character",
                description = "Fetches a character from Visual Novel Database.",
                options = [
                    {
                        name = "character",
                        description = "The character",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
					{
                        name = "search",
                        description = "Whether to search",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
                ],
            },
        ],
    )]
    async fn vndb(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref().unwrap_or("") {
            "visual-novel" => visual_novel::run(input, res).await?,
            "character" => character::run(input, res).await?,
            _ => {},
        }
    }

    vndb
}
