mod load;
mod signups;
mod status;
mod uptime;

use crate::structs::command::Command;
use std::sync::LazyLock;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new()
        .subcommand("load", load::run)
        .subcommand("signups", signups::run)
        .subcommand("status", status::run)
        .subcommand("uptime", uptime::run)
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "heliohost",
        description = "A command for HelioHost.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        subcommands = [
			{
                name = "load",
                description = "Sends HelioHost server load.",
                options = [
                    {
                        name = "server",
                        description = "The server",
                        option_type = InteractionOptionType::STRING,
						choices = [
							ApplicationCommandOptionChoice::new("Tommy", "Tommy"),
							ApplicationCommandOptionChoice::new("Ricky", "Ricky"),
							ApplicationCommandOptionChoice::new("Johnny", "Johnny"),
							ApplicationCommandOptionChoice::new("Lily", "Lily"),
						],
                        required = true,
                    },
                ],
            },
            {
                name = "signups",
                description = "Sends HelioHost server signups status.",
            },
            {
                name = "status",
                description = "Sends HelioHost account status.",
                options = [
                    {
                        name = "user",
                        description = "The user's username or email address",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
                ],
            },
			{
                name = "uptime",
                description = "Sends HelioHost server uptime.",
                options = [
                    {
                        name = "server",
                        description = "The server",
                        option_type = InteractionOptionType::STRING,
						choices = [
							ApplicationCommandOptionChoice::new("Tommy", "Tommy"),
							ApplicationCommandOptionChoice::new("Ricky", "Ricky"),
							ApplicationCommandOptionChoice::new("Johnny", "Johnny"),
							ApplicationCommandOptionChoice::new("Lily", "Lily"),
						],
                        required = true,
                    },
                ],
            },
        ],
    )]
    async fn heliohost(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    heliohost
}
