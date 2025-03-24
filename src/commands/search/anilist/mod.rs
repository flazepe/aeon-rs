mod anime;
mod manga;
mod user;

use crate::structs::{command::AeonCommand, command_context::AeonCommandInput};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("anilist", &["al"]).subcommand("anime", &[], anime::run).subcommand("manga", &[], manga::run).subcommand(
        "user",
        &[],
        user::run,
    )
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Fetches various resources from AniList.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        subcommands = [
			{
                name = "anime",
                description = "Fetches an anime from AniList.",
                options = [
                    {
                        name = "anime",
                        description = "The anime",
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
                name = "manga",
                description = "Fetches a manga from AniList.",
                options = [
                    {
                        name = "manga",
                        description = "The manga",
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
                description = "Fetches a user from AniList.",
                options = [
                    {
                        name = "user",
                        description = "The user",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    }
                ],
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
