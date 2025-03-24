mod image;
mod song;

use crate::structs::{command::AeonCommand, command_context::AeonCommandInput};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> =
    LazyLock::new(|| AeonCommand::new("sauce", &[]).subcommand("image", &[], image::run).subcommand("song", &[], song::run));

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
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
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
