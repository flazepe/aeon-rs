use crate::{
    structs::{api::vndb::Vndb, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::components::SelectOption,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };
    let vndb = Vndb::new();

    if input.is_string_select() {
        return interaction
            .respond(
                vndb.search_trait(&input.values.as_ref().unwrap()[0])
                    .await?
                    .remove(0)
                    .format(),
                false,
            )
            .await;
    }

    let mut results = match vndb.search_trait(input.get_string_arg("trait")?).await {
        Ok(results) => results,
        Err(error) => return interaction.respond_error(error, true).await,
    };

    interaction
        .respond(
            MessageResponse::from(
                SelectMenu::new(
                    "vndb",
                    "trait",
                    "View other results…",
                    results
                        .iter()
                        .map(|result| SelectOption::new(&result.name, &result.id).set_description(&result.group_name))
                        .collect::<Vec<SelectOption>>(),
                    None::<String>,
                )
                .to_components(),
            )
            .add_embed(results.remove(0).format()),
            false,
        )
        .await
}
