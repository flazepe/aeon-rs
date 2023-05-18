use crate::structs::{api::spotify::Spotify, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.get_bool_arg("search").unwrap_or(false) {
        let mut select_menu = SelectMenu::new("spotify", "song", "Select a song…", None::<String>);

        for result in match Spotify::search_track(ctx.get_string_arg("song")?).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        } {
            select_menu = select_menu.add_option(result.name, result.id, Some(&result.artists[0].name))
        }

        return ctx.respond(select_menu, false).await;
    }

    let (query, section): (String, String) = match ctx.input.is_string_select() {
        true => {
            let mut split = ctx.input.values.as_ref().unwrap()[0].split("/");
            (split.next().unwrap().into(), split.next().unwrap_or("").into())
        },
        false => (ctx.get_string_arg("song")?, "".into()),
    };

    let mut track = match ctx.input.is_string_select() {
        true => Spotify::get_track(query).await?,
        false => match Spotify::search_track(query).await {
            Ok(mut result) => result.remove(0),
            Err(error) => return ctx.respond_error(error, true).await,
        },
    };

    ctx.respond(
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
