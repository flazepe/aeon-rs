use crate::{
    structs::{api::saucenao::SauceNAOSearch, interaction::Interaction},
    traits::ArgGetters,
};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
		name = "sauce",
		description = "Fetches sauce from an image.",
		options = [
			{
				name = "image-url",
				description = "The image URL",
				option_type = InteractionOptionType::STRING,
			},
			{
				name = "image-file",
				description = "The image file",
				option_type = InteractionOptionType::ATTACHMENT,
			},
		],
	)]
    async fn sauce(input: CommandInput, res: CommandResponder) {
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        let url = input
            .get_string_arg("image-url")
            .ok()
            .unwrap_or(input.get_attachment_arg("image-file").map_or("".into(), |attachment| attachment.url.to_string()));

        if url.is_empty() {
            return interaction.respond_error("Please provide an image URL or file.", true).await?;
        }

        match SauceNAOSearch::query(url).await {
            Ok(saucenao_search) => interaction.respond(saucenao_search.format(), false).await?,
            Err(error) => interaction.respond_error(error, true).await?,
        };
    }

    sauce
}
