mod load;
mod signups;
mod status;
mod uptime;

use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, InteractionOptionType},
};

pub fn get_command() -> Command {
    #[command(
        name = "heliohost",
        description = "A command for HelioHost.",
        subcommands = [
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
                name = "load",
                description = "Sends HelioHost server load.",
                options = [
                    {
                        name = "server",
                        description = "The server",
                        option_type = InteractionOptionType::STRING,
						choices = [
							ApplicationCommandOptionChoice::new("Cody", "Cody"),
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
        match input.subcommand.as_deref().unwrap_or("") {
            "signups" => signups::run(input, res).await?,
            "status" => status::run(input, res).await?,
            "load" => load::run(input, res).await?,
            "uptime" => uptime::run(input, res).await?,
            _ => {},
        }
    }

    heliohost
}
