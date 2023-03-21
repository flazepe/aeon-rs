use crate::{constants::*, structs::scraping::distrowatch::*, traits::*};
use slashook::{command, commands::*, structs::interactions::*};

pub fn get_command() -> Command {
    #[command(
        name = "distro",
        description = "Fetches a distribution information.",
        options = [
            {
                name = "distro",
                description = "The distribution",
                option_type = InteractionOptionType::STRING,
                required = true
            },
        ],
    )]
    async fn distro(input: CommandInput, res: CommandResponder) {
        match Distro::get(&input.get_string_arg("distro")?).await {
            Ok(distro) => {
                res.send_message(distro.format()).await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        };
    }

    distro
}
