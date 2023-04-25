use crate::{
    structs::{api::vndb::Vndb, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };
    let vndb = Vndb::new();

    if input.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("vndb", "character", "Select a character…", None::<String>);

        for result in match vndb.search_character(input.get_string_arg("character")?).await {
            Ok(results) => results,
            Err(error) => return interaction.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(result.name, result.id, Some(&result.vns[0].title));
        }

        return interaction.respond(select_menu.to_components(), false).await;
    }

    let (query, section): (String, String) = {
        if input.is_string_select() {
            let mut split = input.values.as_ref().unwrap()[0].split("/");
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        } else {
            (input.get_string_arg("character")?, "".into())
        }
    };

    let character = match vndb.search_character(query).await {
        Ok(mut results) => results.remove(0),
        Err(error) => return interaction.respond_error(error, true).await,
    };

    interaction
        .respond(
            MessageResponse::from(match section.as_str() {
                "traits" => character.format_traits(),
                "visual-novels" => character.format_visual_novels(),
                _ => character.format(),
            })
            .set_components(
                SelectMenu::new("vndb", "character", "Select a section…", Some(&section))
                    .add_option("Overview", format!("{}", character.id), None::<String>)
                    .add_option("Traits", format!("{}/traits", character.id), None::<String>)
                    .add_option("Visual Novels", format!("{}/visual-novels", character.id), None::<String>)
                    .to_components(),
            ),
            false,
        )
        .await
}
