mod album;
mod song;

use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
        name = "spotify",
        description = "Fetches various resources from Spotify.",
        subcommands = [
            {
                name = "album",
                description = "Fetches an album from Steam.",
                options = [
                    {
                        name = "album",
                        description = "The album",
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
                name = "song",
                description = "Fetches a song from Steam.",
                options = [
                    {
                        name = "song",
                        description = "The song",
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
    async fn spotify(input: CommandInput, res: CommandResponder) {
        match input.custom_id.as_deref().map_or_else(|| input.subcommand.as_deref().unwrap(), |custom_id| custom_id) {
            "album" => album::run(input, res).await?,
            "song" => song::run(input, res).await?,
            _ => {},
        }
    }

    spotify
}
