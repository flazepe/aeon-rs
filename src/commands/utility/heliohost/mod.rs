mod load;
mod signups;
mod status;
mod uptime;

use crate::structs::{command::AeonCommand, command_context::AeonCommandInput};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static HELIOHOST_SERVERS: [&str; 4] = ["Tommy", "Ricky", "Johnny", "Lily"];
pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("heliohost", &["hh"])
        .add_subcommand("load", &[], load::run)
        .add_subcommand("signups", &[], signups::run)
        .add_subcommand("status", &[], status::run)
        .add_subcommand("uptime", &[], uptime::run)
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
						choices = HELIOHOST_SERVERS
						    .iter()
						    .map(|entry| ApplicationCommandOptionChoice::new(&entry, entry.to_string()))
							.collect::<Vec<ApplicationCommandOptionChoice>>(),
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
						choices = HELIOHOST_SERVERS
						    .iter()
						    .map(|entry| ApplicationCommandOptionChoice::new(&entry, entry.to_string()))
							.collect::<Vec<ApplicationCommandOptionChoice>>(),
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
