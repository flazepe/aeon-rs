mod game;
mod user;

use crate::structs::command::Command;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| Command::new().subcommand("game", game::run).subcommand("user", user::run));

pub fn get_command() -> SlashookCommand {
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
        COMMAND.run(input, res).await?;
    }

    steam
}
