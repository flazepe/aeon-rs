use crate::{
    macros::if_else,
    statics::emojis::ERROR_EMOJI,
    structs::{api::spotify::Spotify, component_interaction::ComponentInteraction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::components::SelectOption,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    if input.get_bool_arg("search").unwrap_or(false) {
        res.send_message(
            SelectMenu::new(
                "spotify",
                "album",
                "Select an album…",
                match Spotify::search_album(input.get_string_arg("album")?).await {
                    Ok(results) => results.into_iter().map(|result| {
                        SelectOption::new(result.name, result.id).set_description(&result.artists[0].name)
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

    let interaction = ComponentInteraction::verify(&input, &res).await?;

    let (query, section): (String, String) = {
        if input.is_string_select() {
            let mut split = input.values.as_ref().unwrap()[0].split("/");
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        } else {
            (input.get_string_arg("album")?, "".into())
        }
    };

    let album = if_else!(
        input.is_string_select(),
        Spotify::get_album(query).await?,
        match Spotify::search_album(query).await {
            Ok(result) => Spotify::get_album(&result[0].id).await?, // Get full album
            Err(error) => return interaction.respond(format!("{ERROR_EMOJI} {error}")).await,
        },
    );

    interaction
        .respond(
            MessageResponse::from(
                SelectMenu::new(
                    "spotify",
                    "album",
                    "Select a section…",
                    vec![
                        SelectOption::new("Overview", format!("{}", album.id)),
                        SelectOption::new("Songs", format!("{}/songs", album.id)),
                        SelectOption::new("Available Countries", format!("{}/available-countries", album.id)),
                    ],
                    Some(&section),
                )
                .to_components(),
            )
            .add_embed(match section.as_str() {
                "songs" => album.format_tracks(),
                "available-countries" => album.format_available_countries(),
                _ => album.format(),
            }),
        )
        .await
}
