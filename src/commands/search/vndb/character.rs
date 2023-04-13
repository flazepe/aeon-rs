use crate::{
    statics::emojis::ERROR_EMOJI,
    structs::{api::vndb::Vndb, component_interaction::ComponentInteraction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::components::SelectOption,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let vndb = Vndb::new();

    if input.get_bool_arg("search")? {
        res.send_message(
            SelectMenu::new(
                "vndb",
                "character",
                "Select a character…",
                match vndb.search_character(input.get_string_arg("character")?).await {
                    Ok(results) => results
                        .into_iter()
                        .map(|mut result| {
                            SelectOption::new(result.name, result.id).set_description(result.vns.remove(0).title)
                        })
                        .collect::<Vec<SelectOption>>(),
                    Err(error) => {
                        res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                        return Ok(());
                    },
                },
                None::<String>,
            )
            .to_components(),
        )
        .await?;

        return Ok(());
    }

    let interaction = ComponentInteraction::verify(&input, &res).await?;

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
        Err(error) => return interaction.respond(format!("{ERROR_EMOJI} {error}")).await,
    };

    interaction
        .respond(
            MessageResponse::from(
                SelectMenu::new(
                    "vndb",
                    "character",
                    "Select a section…",
                    vec![
                        SelectOption::new("Overview", format!("{}", character.id)),
                        SelectOption::new("Traits", format!("{}/traits", character.id)),
                        SelectOption::new("Visual Novels", format!("{}/visual-novels", character.id)),
                    ],
                    Some(&section),
                )
                .to_components(),
            )
            .add_embed(match section.as_str() {
                "traits" => character.format_traits(),
                "visual-novels" => character.format_visual_novels(),
                _ => character.format(),
            }),
        )
        .await
}
