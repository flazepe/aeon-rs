use crate::{
    macros::if_else,
    structs::{api::anilist::AniList, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::channels::Channel,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    if input.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("anilist", "manga", "Select a manga…", None::<String>);

        for result in match AniList::search_manga(input.get_string_arg("manga")?).await {
            Ok(results) => results,
            Err(error) => return interaction.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(
                result.title.romaji,
                result.id,
                Some(format!(
                    "{} - {}",
                    result.format.map_or("TBA".into(), |format| AniList::format_enum_value(format)),
                    AniList::format_enum_value(result.status),
                )),
            );
        }

        return interaction.respond(select_menu.to_components(), false).await;
    }

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
            Err(error) => return interaction.respond_error(error, true).await,
        },
    );

    if manga.is_adult {
        if !Channel::fetch(&input.rest, input.channel_id.as_ref().unwrap()).await.map_or(false, |channel| channel.nsfw.unwrap_or(false)) {
            return interaction.respond_error("NSFW channels only.", true).await;
        }
    }

    interaction
        .respond(
            MessageResponse::from(match section.as_str() {
                "description" => manga.format_description(),
                "characters" => manga.format_characters(),
                "relations" => manga.format_relations(),
                _ => manga.format(),
            })
            .set_components(
                SelectMenu::new("anilist", "manga", "Select a section…", Some(&section))
                    .add_option("Overview", format!("{}", manga.id), None::<String>)
                    .add_option("Description", format!("{}/description", manga.id), None::<String>)
                    .add_option("Characters", format!("{}/characters", manga.id), None::<String>)
                    .add_option("Relations", format!("{}/relations", manga.id), None::<String>)
                    .to_components(),
            ),
            false,
        )
        .await
}
