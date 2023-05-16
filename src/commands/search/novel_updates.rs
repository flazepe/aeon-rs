use crate::{
    structs::{api::localdown::LocalDownNovel, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
		name = "novel-updates",
		description = "Fetches a novel from Novel Updates.",
		options = [
			{
				name = "novel",
				description = "The novel",
				option_type = InteractionOptionType::STRING,
				required = true,
			},
            {
				name = "search",
				description = "Whether to search",
				option_type = InteractionOptionType::BOOLEAN,
			},
		],
	)]
    async fn localdown(input: CommandInput, res: CommandResponder) {
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        if input.get_bool_arg("search").unwrap_or(false) {
            let mut select_menu = SelectMenu::new("novel_updates", "novel_updates", "Select a novel…", None::<String>);

            for result in match LocalDownNovel::search(input.get_string_arg("novel")?).await {
                Ok(results) => results,
                Err(error) => return Ok(interaction.respond_error(error, true).await?),
            } {
                select_menu = select_menu.add_option(result.title, result.id, None::<String>);
            }

            return interaction.respond(select_menu, false).await?;
        }

        interaction
            .respond(
                match input.is_string_select() {
                    true => match LocalDownNovel::get(input.values.as_ref().unwrap()[0].parse::<u64>()?).await {
                        Ok(result) => result,
                        Err(error) => return Ok(interaction.respond_error(error, true).await?),
                    },
                    false => match LocalDownNovel::search(input.get_string_arg("novel")?).await {
                        Ok(results) => LocalDownNovel::get(results[0].id).await?,
                        Err(error) => return Ok(interaction.respond_error(error, true).await?),
                    },
                }
                .format(),
                false,
            )
            .await?;
    }

    localdown
}
