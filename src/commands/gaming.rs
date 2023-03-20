use crate::{
    constants::*,
    structs::api::steam::{game::*, user::*},
    traits::*,
};
use slashook::{command, commands::Command};
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::interactions::*,
};

pub fn get_commands() -> Vec<Command> {
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
                        required = true
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
                        required = true
                    },
                ],
            },
        ]
    )]
    fn steam(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref() {
            Some("game") => match SteamGame::get(&input.get_string_arg("game")?).await {
                Ok(game) => {
                    res.send_message(game.format()).await?;
                }
                Err(error) => {
                    res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                }
            },
            Some("user") => match SteamUser::get(&input.get_string_arg("user")?).await {
                Ok(user) => {
                    res.send_message(user.format()).await?;
                }
                Err(error) => {
                    res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                }
            },
            _ => {}
        }
    }

    vec![steam]
}
