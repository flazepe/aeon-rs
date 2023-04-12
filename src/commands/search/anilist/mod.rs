mod anime;
mod manga;
mod user;

use crate::functions::if_else_option;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
        name = "anilist",
        description = "Fetches various resources from AniList.",
        subcommands = [
			{
                name = "anime",
                description = "Fetches an anime from AniList.",
                options = [
                    {
                        name = "anime",
                        description = "The anime",
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
                name = "manga",
                description = "Fetches a manga from AniList.",
                options = [
                    {
                        name = "manga",
                        description = "The manga",
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
                name = "user",
                description = "Fetches a user from AniList.",
                options = [
                    {
                        name = "user",
                        description = "The user",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    }
                ],
            },
        ],
    )]
    async fn anilist(input: CommandInput, res: CommandResponder) {
        match if_else_option(
            input.custom_id.as_deref(),
            |custom_id| custom_id,
            input.subcommand.as_deref().unwrap_or(""),
        ) {
            "anime" => anime::run(input, res).await?,
            "manga" => manga::run(input, res).await?,
            "user" => user::run(input, res).await?,
            _ => {},
        }
    }

    anilist
}
