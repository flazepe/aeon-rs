use crate::{
    functions::eien,
    structs::{
        api::spotify::Spotify,
        command_context::{CommandContext, CommandInputExt, Input},
        gateway::song_activity::{SongActivity, SongActivityService},
        select_menu::SelectMenu,
    },
    traits::UserExt,
};
use anyhow::Result;
use serde_json::to_string;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if let Input::ApplicationCommand { input, res: _ } = &ctx.input {
        if input.get_bool_arg("search").unwrap_or(false) {
            let results = match Spotify::search_track(input.get_string_arg("song")?).await {
                Ok(results) => results,
                Err(error) => return ctx.respond_error(error, true).await,
            };

            let select_menu = SelectMenu::new("spotify", "song", "Select a song…", None::<String>)
                .add_options(results.iter().map(|result| (&result.name, &result.id, Some(&result.artists[0].name))));

            return ctx.respond(select_menu, false).await;
        }
    }

    let (query, section) = ctx.get_query_and_section("song")?;

    if query.is_empty() {
        return ctx.respond_error("Please provide a song.", true).await;
    }

    let mut track = match ctx.is_string_select() {
        true => Spotify::get_track(query).await?,
        false => match Spotify::search_track(query).await {
            Ok(mut result) => result.remove(0),
            Err(error) => return ctx.respond_error(error, true).await,
        },
    };

    if let Input::ApplicationCommand { input, res: _ } = &ctx.input {
        if let Ok(style) = input.get_string_arg("card").as_deref() {
            let activity = SongActivity {
                service: SongActivityService::Spotify,
                style: style.into(),
                title: track.name,
                artist: track.artists.into_iter().map(|artist| artist.name).collect::<Vec<String>>().join(", "),
                album: track.album.name,
                album_cover: track
                    .album
                    .images
                    .first()
                    .map_or_else(|| input.user.display_avatar_url("png", 4096), |image| image.url.clone()),
                timestamps: None,
            };

            return ctx.respond(eien("song-card", &[&to_string(&activity)?]).await?, false).await;
        }
    }

    let id = &track.id;

    let select_menu = SelectMenu::new("spotify", "song", "View other sections…", Some(&section))
        .add_option("Overview", id, None::<String>)
        .add_option("Audio Features", format!("{id}/audio-features"), None::<String>)
        .add_option("Available Countries", format!("{id}/available-countries"), None::<String>);

    let embed = match section.as_str() {
        "audio-features" => {
            track.get_audio_features().await?;
            track.format_audio_features()
        },
        "available-countries" => track.format_available_countries(),
        _ => track.format(),
    };

    ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
}
