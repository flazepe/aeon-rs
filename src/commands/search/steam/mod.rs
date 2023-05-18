mod game;
mod user;

use crate::structs::command::AeonCommand;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
        name = "steam",
        description = "Fetches various resources from Steam.",
        subcommands = [
            {
                name = "game",
                description = "Fetches a game from Steam.",
                options = [
                    {
                        name = "game",
                        description = "The game",
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
                description = "Fetches a user from Steam.",
                options = [
                    {
                        name = "user",
                        description = "The user",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
                ],
            },
        ],
    )]
    async fn steam(input: CommandInput, res: CommandResponder) {
        AeonCommand::new(input, res).subcommand("game", game::run).subcommand("user", user::run).run().await?;
    }

    steam
}
