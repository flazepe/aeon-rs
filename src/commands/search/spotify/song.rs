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
    if input.get_bool_arg("search")? {
        res.send_message(
            SelectMenu::new(
                "spotify",
                "song",
                "Select a song…",
                match Spotify::search_track(input.get_string_arg("song")?).await {
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
            (input.get_string_arg("song")?, "".into())
        }
    };

    let track = if_else!(
        input.is_string_select(),
        Spotify::get_track(query).await?,
        match Spotify::search_track(query).await {
            Ok(mut result) => result.remove(0),
            Err(error) => return interaction.respond(format!("{ERROR_EMOJI} {error}")).await,
        },
    );

    interaction
        .respond(
            MessageResponse::from(
                SelectMenu::new(
                    "spotify",
                    "song",
                    "Select a section…",
                    vec![
                        SelectOption::new("Overview", format!("{}", track.id)),
                        SelectOption::new("Audio Features", format!("{}/audio-features", track.id)),
                        SelectOption::new("Available Countries", format!("{}/available-countries", track.id)),
                    ],
                    Some(&section),
                )
                .to_components(),
            )
            .add_embed(match section.as_str() {
                "audio-features" => track.get_audio_features().await?.format_audio_features(),
                "available-countries" => track.format_available_countries(),
                _ => track.format(),
            }),
        )
        .await
}
