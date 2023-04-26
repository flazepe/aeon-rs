use crate::{
    structs::{api::jisho::JishoSearch, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::InteractionOptionType,
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
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        if input.is_string_select() {
            return interaction.respond(JishoSearch::get(&input.values.as_ref().unwrap()[0]).await?.format(), false).await?;
        }

        let results = match JishoSearch::search(input.get_string_arg("query")?).await {
            Ok(results) => results,
            Err(error) => return Ok(interaction.respond_error(error, true).await?),
        };

        let mut select_menu = SelectMenu::new("jisho", "search", "View other results…", None::<String>);

        for result in &results {
            select_menu = select_menu.add_option(result.format_title(), result.slug.clone(), None::<String>);
        }

        interaction.respond(MessageResponse::from(select_menu).add_embed(results[0].format()), false).await?;
    }

    jisho
}
