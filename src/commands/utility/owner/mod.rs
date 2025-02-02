mod eien;
mod request;
mod status;

use crate::structs::command::Command;
use std::sync::LazyLock;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().owner_only().subcommand("eien", eien::run).subcommand("request", request::run).subcommand("status", status::run)
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "owner",
        description = "Owner commands.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        subcommands = [
            {
                name = "eien",
                description = "For testing Eien.",
				options = [
					{
                        name = "command",
                        description = "The command",
                        option_type = InteractionOptionType::STRING,
						required = true,
                    },
					{
                        name = "args",
                        description = "The args",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
				],
            },
			{
                name = "request",
                description = "Creates a request to the Discord API.",
				options = [
					{
                        name = "endpoint",
                        description = "The endpoint",
                        option_type = InteractionOptionType::STRING,
						required = true,
                    },
					{
                        name = "body",
                        description = "The body",
                        option_type = InteractionOptionType::STRING,
                    },
					{
                        name = "method",
                        description = "The HTTP method",
                        option_type = InteractionOptionType::STRING,
						choices = [
							ApplicationCommandOptionChoice::new("GET", "GET"),
							ApplicationCommandOptionChoice::new("POST", "POST"),
							ApplicationCommandOptionChoice::new("PUT", "PUT"),
							ApplicationCommandOptionChoice::new("DELETE", "DELETE"),
							ApplicationCommandOptionChoice::new("HEAD", "HEAD"),
							ApplicationCommandOptionChoice::new("OPTIONS", "OPTIONS"),
							ApplicationCommandOptionChoice::new("CONNECT", "CONNECT"),
							ApplicationCommandOptionChoice::new("PATCH", "PATCH"),
							ApplicationCommandOptionChoice::new("TRACE", "TRACE"),
						],
                    },
				],
            },
            {
                name = "status",
                description = "Sends the process status.",
            },
        ],
    )]
    async fn owner(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    owner
}
