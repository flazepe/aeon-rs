mod cdpt;
mod cleanurl;
mod isgd;
mod waaai;
mod zws;

use crate::structs::command::Command;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new()
        .subcommand("cdpt", cdpt::run)
        .subcommand("cleanurl", cleanurl::run)
        .subcommand("isgd", isgd::run)
        .subcommand("waaai", waaai::run)
        .subcommand("zws", zws::run)
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "shorten-url",
        description = "Shortens a URL using different services.",
        subcommands = [
            {
                name = "cdpt",
                description = "Uses cdpt.in URL shortener.",
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
                description = "Uses cleanuri.com URL shortener.",
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
                name = "isgd",
                description = "Uses is.gd URL shortener.",
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
                        name = "lowercase",
                        description = "Whether to generate lowercased shortened URL",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
					{
                        name = "pronounceable",
                        description = "Whether to generate pronounceable shortened URL",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
                ],
            },
			{
                name = "waaai",
                description = "Uses waa.ai URL shortener.",
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
                ],
            },
			{
                name = "zws",
                description = "Uses zws.im URL shortener.",
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
