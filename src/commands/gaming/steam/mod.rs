pub mod game;
pub mod user;

use slashook::{command, commands::Command};
use slashook::{
    commands::{CommandInput, CommandResponder},
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
        match input.subcommand.as_deref().unwrap_or("") {
            "game" => game::run(input, res).await?,
            "user" => user::run(input, res).await?,
            _ => {},
        }
    }

    steam
}
