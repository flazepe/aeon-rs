mod character;
mod visual_novel;

use crate::macros::{and_then_or, verify_component_interaction};
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
        verify_component_interaction!(input, res);

        match and_then_or!(
            input.custom_id.as_deref(),
            |custom_id| Some(custom_id),
            input.subcommand.as_deref().unwrap_or("")
        ) {
            "visual-novel" => return visual_novel::run(input, res).await?,
            "character" => character::run(input, res).await?,
            _ => {},
        }
    }

    vndb
}
