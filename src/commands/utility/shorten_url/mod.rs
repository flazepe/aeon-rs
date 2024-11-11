mod cdpt;
mod cleanurl;
mod waaai;
mod zws;

use crate::structs::command::Command;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new()
        .subcommand("cdpt", cdpt::run)
        .subcommand("cleanurl", cleanurl::run)
        .subcommand("waaai", waaai::run)
        .subcommand("zws", zws::run)
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "shorten-url",
        description = "Shortens a URL using different services.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        subcommands = [
            {
                name = "cdpt",
                description = "Shortens a URL using cdpt.in.",
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
    async fn shorten_url(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    shorten_url
}
