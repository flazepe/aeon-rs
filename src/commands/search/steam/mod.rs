mod game;
mod user;

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
        match input
            .custom_id
            .as_deref()
            .map_or_else(|| input.subcommand.as_deref().unwrap(), |custom_id| custom_id)
        {
            "game" => game::run(input, res).await?,
            "user" => user::run(input, res).await?,
            _ => {},
        }
    }

    steam
}
