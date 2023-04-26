use crate::{
    structs::{api::spotify::Spotify, interaction::Interaction, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    if input.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("spotify", "song", "Select a song…", None::<String>);

        for result in match Spotify::search_track(input.get_string_arg("song")?).await {
            Ok(results) => results,
            Err(error) => return interaction.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(result.name, result.id, Some(&result.artists[0].name))
        }

        return interaction.respond(select_menu, false).await;
    }

    let (query, section): (String, String) = match input.is_string_select() {
        true => {
            let mut split = input.values.as_ref().unwrap()[0].split("/");
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (input.get_string_arg("song")?, "".into()),
    };

    let mut track = match input.is_string_select() {
        true => Spotify::get_track(query).await?,
        false => match Spotify::search_track(query).await {
            Ok(mut result) => result.remove(0),
            Err(error) => return interaction.respond_error(error, true).await,
        },
    };

    interaction
        .respond(
            MessageResponse::from(
                SelectMenu::new("spotify", "song", "Select a section…", Some(&section))
                    .add_option("Overview", format!("{}", track.id), None::<String>)
                    .add_option("Audio Features", format!("{}/audio-features", track.id), None::<String>)
                    .add_option("Available Countries", format!("{}/available-countries", track.id), None::<String>),
            )
            .add_embed(match section.as_str() {
                "audio-features" => {
                    track.get_audio_features().await?;
                    track.format_audio_features()
                },
                "available-countries" => track.format_available_countries(),
                _ => track.format(),
            }),
            false,
        )
        .await
}
