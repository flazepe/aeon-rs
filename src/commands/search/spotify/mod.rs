mod album;
mod lyrics;
mod member;
mod song;

use crate::structs::command::Command;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new()
        .subcommand("album", album::run)
        .subcommand("lyrics", lyrics::run)
        .subcommand("member", member::run)
        .subcommand("song", song::run)
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "spotify",
        description = "Fetches various resources from Spotify.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
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
                name = "lyrics",
                description = "Fetches song lyrics based on query or user's Spotify status.",
                options = [
                    {
                        name = "song",
                        description = "The song",
                        option_type = InteractionOptionType::STRING,
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
                    {
                        name = "card",
                        description = "Whether to send a card instead. This would be the card style",
                        option_type = InteractionOptionType::STRING,
                        choices = [
                            ApplicationCommandOptionChoice::new("Classic", "classic"),
                            ApplicationCommandOptionChoice::new("Modern", "modern"),
                            ApplicationCommandOptionChoice::new("Nori", "nori"),
                            ApplicationCommandOptionChoice::new("Rovi", "rovi"),
                            ApplicationCommandOptionChoice::new("VXC", "vxc"),
                        ],
                    },
                ],
            },
            {
                name = "member",
                description = "Generates a card from a member's Spotify activity.",
                options = [
                    {
                        name = "member",
                        description = "The member to show the card for",
                        option_type = InteractionOptionType::USER,
                    },
                    {
                        name = "card",
                        description = "The card style",
                        option_type = InteractionOptionType::STRING,
                        choices = [
                            ApplicationCommandOptionChoice::new("Classic", "classic"),
                            ApplicationCommandOptionChoice::new("Modern", "modern"),
                            ApplicationCommandOptionChoice::new("Nori", "nori"),
                            ApplicationCommandOptionChoice::new("Rovi", "rovi"),
                            ApplicationCommandOptionChoice::new("VXC", "vxc"),
                        ],
                    },
                    {
                        name = "collapse",
                        description = "Whether to collapse the card for supported ones",
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
