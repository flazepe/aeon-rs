mod character;
mod character_trait;
mod tag;
mod visual_novel;

use crate::structs::command::AeonCommand;
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
            {
                name = "tag",
                description = "Fetches a tag from Visual Novel Database.",
                options = [
                    {
                        name = "tag",
                        description = "The tag",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
                ],
            },
            {
                name = "trait",
                description = "Fetches a trait from Visual Novel Database.",
                options = [
                    {
                        name = "trait",
                        description = "The trait",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
                ],
            },
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
        ],
    )]
    async fn vndb(input: CommandInput, res: CommandResponder) {
        AeonCommand::new(input, res)
            .subcommand("character", character::run)
            .subcommand("tag", tag::run)
            .subcommand("trait", character_trait::run)
            .subcommand("visual-novel", visual_novel::run)
            .run()
            .await?;
    }

    vndb
}
