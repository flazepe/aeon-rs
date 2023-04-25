use crate::{
    macros::if_else,
    structs::{api::spotify::Spotify, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    if input.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("spotify", "album", "Select an album…", None::<String>);

        for result in match Spotify::search_album(input.get_string_arg("album")?).await {
            Ok(results) => results,
            Err(error) => return interaction.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(result.name, result.id, Some(&result.artists[0].name))
        }

        return interaction.respond(select_menu.to_components(), false).await;
    }

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
            Err(error) => return interaction.respond_error(error, true).await,
        },
    );

    interaction
        .respond(
            MessageResponse::from(match section.as_str() {
                "songs" => album.format_tracks(),
                "available-countries" => album.format_available_countries(),
                _ => album.format(),
            })
            .set_components(
                SelectMenu::new("spotify", "album", "Select a section…", Some(&section))
                    .add_option("Overview", format!("{}", album.id), None::<String>)
                    .add_option("Songs", format!("{}/songs", album.id), None::<String>)
                    .add_option("Available Countries", format!("{}/available-countries", album.id), None::<String>)
                    .to_components(),
            ),
            false,
        )
        .await
}
