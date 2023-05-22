mod load;
mod signups;
mod status;
mod uptime;

use crate::structs::command::Command;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
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
