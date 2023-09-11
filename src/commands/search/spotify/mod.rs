mod album;
mod card;
mod song;

use crate::structs::command::Command;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, InteractionOptionType},
};

static COMMAND: Lazy<Command> =
    Lazy::new(|| Command::new().subcommand("album", album::run).subcommand("card", card::run).subcommand("song", song::run));

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "spotify",
        description = "Fetches various resources from Spotify.",
        subcommands = [
            {
                name = "album",
                description = "Fetches an album from Spotify.",
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
                description = "Fetches a song from Spotify.",
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
            {
                name = "card",
                description = "Generates a card from a member's Spotify activity.",
                options = [
                    {
                        name = "member",
                        description = "The member to show the card for",
                        option_type = InteractionOptionType::USER,
                    },
                    {
                        name = "style",
                        description = "The card style",
                        option_type = InteractionOptionType::STRING,
                        choices = [
                            ApplicationCommandOptionChoice::new("Classic", "classic"),
                            ApplicationCommandOptionChoice::new("Nori", "nori"),
                            ApplicationCommandOptionChoice::new("Rovi", "rovi"),
                            ApplicationCommandOptionChoice::new("VXC", "vxc"),
                        ],
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
