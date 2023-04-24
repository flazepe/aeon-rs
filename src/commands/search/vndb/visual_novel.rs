use crate::{
    statics::emojis::ERROR_EMOJI,
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

    if input.get_bool_arg("search").unwrap_or(false) {
        return interaction
            .respond(
                SelectMenu::new(
                    "vndb",
                    "visual-novel",
                    "Select a visual novel…",
                    match vndb.search_visual_novel(input.get_string_arg("visual-novel")?).await {
                        Ok(results) => results
                            .into_iter()
                            .map(|result| SelectOption::new(result.title, result.id).set_description(result.dev_status))
                            .collect::<Vec<SelectOption>>(),
                        Err(error) => return interaction.respond_error(error, true).await,
                    },
                    None::<String>,
                )
                .to_components(),
                false,
            )
            .await;
    }

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
        Err(error) => return interaction.respond(format!("{ERROR_EMOJI} {error}"), true).await,
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
            false,
        )
        .await
}
