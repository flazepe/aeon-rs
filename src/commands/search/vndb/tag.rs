use crate::{
    structs::{api::vndb::Vndb, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };
    let vndb = Vndb::new();

    if input.is_string_select() {
        return interaction.respond(vndb.search_tag(&input.values.as_ref().unwrap()[0]).await?.remove(0).format(), false).await;
    }

    let results = match vndb.search_tag(input.get_string_arg("tag")?).await {
        Ok(results) => results,
        Err(error) => return interaction.respond_error(error, true).await,
    };

    let mut select_menu = SelectMenu::new("vndb", "tag", "View other results…", None::<String>);

    for result in &results {
        select_menu = select_menu.add_option(&result.name, &result.id, Some(&result.category))
    }

    interaction.respond(MessageResponse::from(results[0].format()).set_components(select_menu.to_components()), false).await
}
