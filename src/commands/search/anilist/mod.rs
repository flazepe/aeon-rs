mod anime;
mod manga;

use crate::macros::and_then_or;
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
                ],
            },
        ],
    )]
    async fn anilist(input: CommandInput, res: CommandResponder) {
        match and_then_or!(
            input.custom_id.as_deref(),
            |custom_id| Some(custom_id),
            input.subcommand.as_deref().unwrap_or("")
        ) {
            "anime" => anime::run(input, res).await?,
            "manga" => manga::run(input, res).await?,
            _ => {},
        }
    }

    anilist
}
