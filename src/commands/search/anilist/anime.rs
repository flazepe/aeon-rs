use crate::{
    functions::if_else_option,
    macros::if_else,
    statics::emojis::ERROR_EMOJI,
    structs::{api::anilist::AniList, restricted_interaction::RestrictedInteraction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::components::SelectOption,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    if input.get_bool_arg("search")? {
        res.send_message(
            SelectMenu::new(
                "anilist",
                "anime",
                "Select an anime…",
                match AniList::search_anime(input.get_string_arg("anime")?).await {
                    Ok(results) => results.into_iter().map(|result| {
                        SelectOption::new(result.title.romaji, result.id).set_description(format!(
                            "{} - {}",
                            if_else_option(result.format, |format| AniList::format_enum_value(format), "TBA".into(),),
                            AniList::format_enum_value(result.status),
                        ))
                    }),
                    Err(error) => {
                        res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                        return Ok(());
                    },
                }
                .collect::<Vec<SelectOption>>(),
                None::<String>,
            )
            .to_components(),
        )
        .await?;

        return Ok(());
    }

    let interaction = RestrictedInteraction::verify(&input, &res).await?;

    let (query, section): (String, String) = {
        if input.is_string_select() {
            let mut split = input.values.as_ref().unwrap()[0].split("/");
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        } else {
            (input.get_string_arg("anime")?, "".into())
        }
    };

    let anime = if_else!(
        input.is_string_select(),
        AniList::get_anime(query.parse::<u64>()?).await?,
        match AniList::search_anime(query).await {
            Ok(mut results) => results.remove(0),
            Err(error) => return interaction.respond(format!("{ERROR_EMOJI} {error}")).await,
        },
    );

    interaction
        .respond(
            MessageResponse::from(
                SelectMenu::new(
                    "anilist",
                    "anime",
                    "Select a section…",
                    vec![
                        SelectOption::new("Overview", format!("{}", anime.id)),
                        SelectOption::new("Description", format!("{}/description", anime.id)),
                        SelectOption::new("Characters", format!("{}/characters", anime.id)),
                        SelectOption::new("Relations", format!("{}/relations", anime.id)),
                    ],
                    Some(&section),
                )
                .to_components(),
            )
            .add_embed(match section.as_str() {
                "description" => anime.format_description(),
                "characters" => anime.format_characters(),
                "relations" => anime.format_relations(),
                _ => anime.format(),
            }),
        )
        .await
}
