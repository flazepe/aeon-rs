use crate::{
    structs::{interaction::Interaction, scraping::distrowatch::Distro},
    traits::ArgGetters,
};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
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
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        match Distro::get(input.get_string_arg("distro")?).await {
            Ok(distro) => interaction.respond(distro.format(), false).await?,
            Err(error) => interaction.respond_error(error, true).await?,
        };
    }

    distrowatch
}
