use crate::{statics::emojis::ERROR_EMOJI, structs::api::saucenao::SauceNAOSearch, traits::ArgGetters};
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
				name = "image-attachment",
				description = "The image attachment",
				option_type = InteractionOptionType::ATTACHMENT,
			},
		],
	)]
    async fn sauce(input: CommandInput, res: CommandResponder) {
        let url = input.get_string_arg("image-url").ok().unwrap_or(
            input
                .get_attachment_arg("image-attachment")
                .map_or("".into(), |attachment| attachment.url.to_string()),
        );

        if url.is_empty() {
            return res
                .send_message(format!("{ERROR_EMOJI} Please provide an image URL or attachment."))
                .await?;
        }

        match SauceNAOSearch::query(url).await {
            Ok(saucenao_search) => res.send_message(saucenao_search.format()).await?,
            Err(error) => res.send_message(format!("{ERROR_EMOJI} {error}")).await?,
        };
    }

    sauce
}
