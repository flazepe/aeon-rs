mod album;
mod song;

use crate::structs::command::AeonCommand;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| AeonCommand::new().subcommand("album", album::run).subcommand("song", song::run));

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
        COMMAND.run(input, res).await?;
    }

    spotify
}
