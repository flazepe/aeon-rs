mod image;
mod song;

use crate::structs::command::Command;
use std::sync::LazyLock;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: LazyLock<Command> = LazyLock::new(|| Command::new().subcommand("image", image::run).subcommand("song", song::run));

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "sauce",
		description = "Fetches sauce from an image or song.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
		subcommands = [
            {
                name = "image",
                description = "Fetches sauce from an image.",
                options = [
                    {
                        name = "image-url",
                        description = "The image URL",
                        option_type = InteractionOptionType::STRING,
                    },
                    {
                        name = "image-file",
                        description = "The image file",
                        option_type = InteractionOptionType::ATTACHMENT,
                    },
                ],
            },
            {
                name = "song",
                description = "Fetches sauce from a song title or lyrics.",
                options = [
                    {
                        name = "song",
                        description = "The song title or partial lyrics",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
                ],
            },
        ]
	)]
    async fn sauce(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    sauce
}
