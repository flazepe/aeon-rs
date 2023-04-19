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

    if input.get_bool_arg("search").unwrap_or(false) {
        res.send_message(
            SelectMenu::new(
                "vndb",
                "visual-novel",
                "Select a visual novel…",
                match vndb.search_visual_novel(input.get_string_arg("visual-novel")?).await {
                    Ok(results) => results
                        .into_iter()
                        .map(|result| SelectOption::new(result.title, result.id).set_description(result.dev_status))
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

    let Ok(interaction) = ComponentInteraction::verify(&input, &res).await else { return Ok(()); };

    let (query, section): (String, String) = {
        if input.is_string_select() {
            let mut split = input.values.as_ref().unwrap()[0].split("/");
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        } else {
            (input.get_string_arg("visual-novel")?, "".into())
        }
    };

    let visual_novel = match vndb.search_visual_novel(query).await {
        Ok(mut results) => results.remove(0),
        Err(error) => return interaction.respond(format!("{ERROR_EMOJI} {error}")).await,
    };

    interaction
        .respond(
            MessageResponse::from(
                SelectMenu::new(
                    "vndb",
                    "visual-novel",
                    "Select a section…",
                    vec![
                        SelectOption::new("Overview", format!("{}", visual_novel.id)),
                        SelectOption::new("Description", format!("{}/description", visual_novel.id)),
                        SelectOption::new("Tags", format!("{}/tags", visual_novel.id)),
                    ],
                    Some(&section),
                )
                .to_components(),
            )
            .add_embed(match section.as_str() {
                "description" => visual_novel.format_description(),
                "tags" => visual_novel.format_tags(),
                _ => visual_novel.format(),
            }),
        )
        .await
}
