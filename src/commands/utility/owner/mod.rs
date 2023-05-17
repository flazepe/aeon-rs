mod request;
mod status;

use crate::statics::FLAZEPE_ID;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, InteractionOptionType},
};

pub fn get_command() -> Command {
    #[command(
        name = "owner",
        description = "Owner commands.",
        subcommands = [
            {
                name = "status",
                description = "Sends the process status.",
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
        ],
    )]
    async fn code(input: CommandInput, res: CommandResponder) {
        if input.user.id != FLAZEPE_ID {
            return res.send_message("No").await?;
        }

        match input.custom_id.as_deref().map_or_else(|| input.subcommand.as_deref().unwrap(), |custom_id| custom_id) {
            "status" => status::run(input, res).await?,
            "request" => request::run(input, res).await?,
            _ => {},
        };
    }

    code
}
