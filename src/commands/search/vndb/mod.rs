mod character;
mod character_trait;
mod tag;
mod visual_novel;

use crate::structs::{command::Command, command_context::Input};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("vndb", &[])
        .subcommand("character", &[], character::run)
        .subcommand("tag", &[], tag::run)
        .subcommand("trait", &[], character_trait::run)
        .subcommand("visual-novel", &["vn"], visual_novel::run)
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Searches for various resources from Visual Novel Database.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        subcommands = [
            {
                name = "character",
                description = "Fetches a character from Visual Novel Database.",
                options = [
                    {
                        name = "character",
                        description = "The character",
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
                name = "tag",
                description = "Fetches a tag from Visual Novel Database.",
                options = [
                    {
                        name = "tag",
                        description = "The tag",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
                ],
            },
            {
                name = "trait",
                description = "Fetches a trait from Visual Novel Database.",
                options = [
                    {
                        name = "trait",
                        description = "The trait",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
                ],
            },
            {
                name = "visual-novel",
                description = "Fetches a visual novel from Visual Novel Database.",
                options = [
                    {
                        name = "visual-novel",
                        description = "The visual novel",
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
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(Input::ApplicationCommand { input, res }).await?;
    }

    func
}
