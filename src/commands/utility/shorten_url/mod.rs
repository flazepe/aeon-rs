mod cleanurl;
mod waaai;
mod zws;

use crate::structs::{command::AeonCommand, command_context::AeonCommandInput};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("shorten-url", &["shorten", "url"])
        .add_subcommand("cleanurl", &[], cleanurl::run)
        .add_subcommand("waaai", &[], waaai::run)
        .add_subcommand("zws", &[], zws::run)
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Shortens a URL using different services.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        subcommands = [
			{
                name = "cleanurl",
                description = "Shortens a URL using cleanuri.com.",
                options = [
                    {
                        name = "url",
                        description = "The URL",
                        option_type = InteractionOptionType::STRING,
						required = true,
                    },
                ],
            },
			{
                name = "waaai",
                description = "Shortens a URL using waa.ai.",
                options = [
                    {
                        name = "url",
                        description = "The URL",
                        option_type = InteractionOptionType::STRING,
						required = true,
                    },
					{
                        name = "custom-id",
                        description = "The custom ID",
                        option_type = InteractionOptionType::STRING,
                    },
                    {
                        name = "hash",
                        description = "Whether to include a hash after the ID. Should not be used with a custom ID",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
                ],
            },
			{
                name = "zws",
                description = "Shortens a URL using zws.im.",
                options = [
                    {
                        name = "url",
                        description = "The URL",
                        option_type = InteractionOptionType::STRING,
						required = true,
                    },
                ],
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
