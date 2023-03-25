use crate::{statics::emojis::*, traits::*, *};
use reqwest::get;
use slashook::{command, commands::*, structs::interactions::*};

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
    fn snipe(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref().unwrap_or("") {
            "signups" => {
                res.send_message("TODO").await?;
            }
            "status" => {
                res.send_message("TODO").await?;
            }
            "load" => {
                let server = input.get_string_arg("server")?;

                res.send_message(format!(
                    "{}'s load is `{}`.",
                    server,
                    get(format!("https://heliohost.org/load/load_{server}.html").to_lowercase())
                        .await?
                        .text()
                        .await?
                        .trim()
                ))
                .await?;
            }
            "uptime" => {
                let server = input.get_string_arg("server")?;

                res.send_message(format!(
                    "{}'s uptime is `{}`.",
                    server,
                    get(format!("https://heliohost.org/load/uptime_{server}.html").to_lowercase())
                        .await?
                        .text()
                        .await?
                        .trim()
                ))
                .await?;
            }
            _ => {}
        }
    }

    snipe
}
