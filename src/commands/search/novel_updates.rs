use crate::{
    structs::{api::localdown::LocalDownNovel, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, MessageResponse},
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
		],
	)]
    async fn localdown(input: CommandInput, res: CommandResponder) {
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        if input.is_string_select() {
            return match LocalDownNovel::get(input.values.as_ref().unwrap()[0].parse::<u64>()?).await {
                Ok(result) => interaction.respond(result.format(), false).await?,
                Err(error) => interaction.respond_error(error, true).await?,
            };
        }

        let results = match LocalDownNovel::search(input.get_string_arg("novel")?).await {
            Ok(results) => results,
            Err(error) => return Ok(interaction.respond_error(error, true).await?),
        };

        let mut select_menu = SelectMenu::new("novel-updates", "novel-updates", "Select a novel…", None::<String>);

        for result in &results {
            select_menu = select_menu.add_option(&result.title, result.id, None::<String>);
        }

        interaction
            .respond(
                MessageResponse::from(select_menu).add_embed(match LocalDownNovel::get(results[0].id).await {
                    Ok(result) => result.format(),
                    Err(error) => return Ok(interaction.respond_error(error, true).await?),
                }),
                false,
            )
            .await?;
    }

    localdown
}
