mod load;
mod signups;
mod status;
mod uptime;

use crate::structs::{command::Command, command_context::Input};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("heliohost", &["hh"])
        .subcommand("load", &[], load::run)
        .subcommand("signups", &[], signups::run)
        .subcommand("status", &[], status::run)
        .subcommand("uptime", &[], uptime::run)
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
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
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(Input::ApplicationCommand(input, res)).await?;
    }

    func
}
