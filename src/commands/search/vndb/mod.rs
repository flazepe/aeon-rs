mod character;
mod character_trait;
mod tag;
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
        match input
            .custom_id
            .as_deref()
            .map_or_else(|| input.subcommand.as_deref().unwrap_or(""), |custom_id| custom_id)
        {
            "character" => character::run(input, res).await?,
            "tag" => tag::run(input, res).await?,
            "trait" => character_trait::run(input, res).await?,
            "visual-novel" => visual_novel::run(input, res).await?,
            _ => {},
        }
    }

    vndb
}
