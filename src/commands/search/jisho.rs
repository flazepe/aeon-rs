use crate::{
    statics::emojis::ERROR_EMOJI,
    structs::{api::jisho::JishoSearch, restricted_interaction::RestrictedInteraction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, MessageResponse},
    structs::{components::SelectOption, interactions::InteractionOptionType},
};

pub fn get_command() -> Command {
    #[command(
		name = "jisho",
		description = "Searches Jisho.",
		options = [
			{
				name = "query",
				description = "The query",
				option_type = InteractionOptionType::STRING,
				required = true,
			},
		],
	)]
    async fn jisho(input: CommandInput, res: CommandResponder) {
        if input.is_string_select() {
            return RestrictedInteraction::verify(&input, &res)
                .await?
                .respond(JishoSearch::get(&input.values.as_ref().unwrap()[0]).await?.format())
                .await?;
        }

        let mut results = match JishoSearch::search(input.get_string_arg("query")?).await {
            Ok(results) => results,
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                return Ok(());
            },
        };

        res.send_message(
            MessageResponse::from(
                SelectMenu::new(
                    "jisho",
                    "search",
                    "View other results…",
                    results
                        .iter()
                        .map(|result| SelectOption::new(result.format_title(), result.slug.clone()))
                        .collect::<Vec<SelectOption>>(),
                    None::<String>,
                )
                .to_components(),
            )
            .add_embed(results.remove(0).format()),
        )
        .await?;
    }

    jisho
}
