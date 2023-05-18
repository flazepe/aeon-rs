mod anime;
mod manga;
mod user;

use crate::structs::command::AeonCommand;
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
        AeonCommand::new(input, res)
            .subcommand("anime", anime::run)
            .subcommand("manga", manga::run)
            .subcommand("user", user::run)
            .run()
            .await?;
    }

    anilist
}
