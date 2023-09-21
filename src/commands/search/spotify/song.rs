use crate::{
    functions::eien,
    structs::{
        api::spotify::Spotify,
        command_context::CommandContext,
        gateway::song_activity::{SongActivity, SongActivityService},
        select_menu::SelectMenu,
    },
    traits::AvatarUrl,
};
use anyhow::Result;
use serde_json::to_string;
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
            let mut split = ctx.input.values.as_ref().unwrap()[0].split('/');
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

    if let Ok(style) = ctx.get_string_arg("card").as_deref() {
        ctx.res.defer(false).await?;

        return ctx
            .respond(
                eien(
                    "song-card",
                    &[&to_string(&SongActivity {
                        service: SongActivityService::Spotify,
                        style: style.into(),
                        title: track.name,
                        artist: track.artists.into_iter().map(|artist| artist.name).collect::<Vec<String>>().join(", "),
                        album: track.album.name,
                        album_cover: track
                            .album
                            .images
                            .get(0)
                            .map_or_else(|| ctx.input.user.display_avatar_url("png", 4096), |image| image.url.clone()),
                        timestamps: None,
                    })?],
                )
                .await?,
                false,
            )
            .await;
    }

    ctx.respond(
        MessageResponse::from(
            SelectMenu::new("spotify", "song", "Select a section…", Some(&section))
                .add_option("Overview", &track.id, None::<String>)
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
