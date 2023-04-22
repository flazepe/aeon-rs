use crate::{
    macros::if_else,
    statics::emojis::ERROR_EMOJI,
    structs::{api::anilist::AniList, component_interaction::ComponentInteraction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::{channels::Channel, components::SelectOption},
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    if input.get_bool_arg("search").unwrap_or(false) {
        res.send_message(
            SelectMenu::new(
                "anilist",
                "manga",
                "Select a manga…",
                match AniList::search_manga(input.get_string_arg("manga")?).await {
                    Ok(results) => results
                        .into_iter()
                        .map(|result| {
                            SelectOption::new(result.title.romaji, result.id).set_description(format!(
                                "{} - {}",
                                result
                                    .format
                                    .map_or("TBA".into(), |format| AniList::format_enum_value(format)),
                                AniList::format_enum_value(result.status),
                            ))
                        })
                        .collect::<Vec<SelectOption>>(),
                    Err(error) => {
                        res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} {error}")).set_ephemeral(true))
                            .await?;

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
            (input.get_string_arg("manga")?, "".into())
        }
    };

    let manga = if_else!(
        input.is_string_select(),
        AniList::get_manga(query.parse::<u64>()?).await?,
        match AniList::search_manga(query).await {
            Ok(mut results) => results.remove(0),
            Err(error) => return interaction.respond(format!("{ERROR_EMOJI} {error}")).await,
        },
    );

    if manga.is_adult {
        if !Channel::fetch(&input.rest, input.channel_id.as_ref().unwrap())
            .await
            .map_or(false, |channel| channel.nsfw.unwrap_or(false))
        {
            return interaction.respond(format!("{ERROR_EMOJI} NSFW channels only.")).await;
        }
    }

    interaction
        .respond(
            MessageResponse::from(
                SelectMenu::new(
                    "anilist",
                    "manga",
                    "Select a section…",
                    vec![
                        SelectOption::new("Overview", format!("{}", manga.id)),
                        SelectOption::new("Description", format!("{}/description", manga.id)),
                        SelectOption::new("Characters", format!("{}/characters", manga.id)),
                        SelectOption::new("Relations", format!("{}/relations", manga.id)),
                    ],
                    Some(&section),
                )
                .to_components(),
            )
            .add_embed(match section.as_str() {
                "description" => manga.format_description(),
                "characters" => manga.format_characters(),
                "relations" => manga.format_relations(),
                _ => manga.format(),
            }),
        )
        .await
}
