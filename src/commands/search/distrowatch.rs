use crate::{statics::emojis::ERROR_EMOJI, structs::scraping::distrowatch::Distro, traits::ArgGetters};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
        name = "distrowatch",
        description = "Fetches a distribution from distrowatch.",
        options = [
            {
                name = "distro",
                description = "The distribution",
                option_type = InteractionOptionType::STRING,
                required = true,
            },
        ],
    )]
    async fn distrowatch(input: CommandInput, res: CommandResponder) {
        match Distro::get(input.get_string_arg("distro")?).await {
            Ok(distro) => {
                res.send_message(distro.format()).await?;
            },
            Err(error) => {
                res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} {error}")).set_ephemeral(true))
                    .await?;
            },
        };
    }

    distrowatch
}
