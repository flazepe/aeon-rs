mod request;
mod status;

use crate::structs::command::Command;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, InteractionOptionType},
};

static COMMAND: Lazy<Command> =
    Lazy::new(|| Command::new().owner_only().subcommand("request", request::run).subcommand("status", status::run));

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "owner",
        description = "Owner commands.",
        subcommands = [
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
    async fn code(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    code
}
