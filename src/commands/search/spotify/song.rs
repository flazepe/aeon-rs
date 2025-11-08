use crate::{
    functions::eien,
    structs::{
        api::spotify::Spotify,
        command_context::{AeonCommandContext, AeonCommandInput},
        gateway::song_activity::{SongActivity, SongActivityService},
        select_menu::SelectMenu,
    },
};
use anyhow::Result;
use serde_json::to_string;
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(..) = &ctx.command_input
        && ctx.get_bool_arg("search").unwrap_or(false)
    {
        let results = Spotify::search_track(ctx.get_string_arg("song", 0, true)?).await?;
        let options = results.iter().map(|result| (&result.name, &result.id, Some(&result.artists[0].name)));
        let select_menu = SelectMenu::new("spotify", "song", "Select a song…", None::<String>).add_options(options);

        return ctx.respond(select_menu, false).await;
    }

    let (query, section) = ctx.get_query_and_section("song")?;
    let mut track = if ctx.is_string_select() { Spotify::get_track(query).await? } else { Spotify::search_track(query).await?.remove(0) };

    if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input
        && let Ok(style) = ctx.get_string_arg("card", 0, true).as_deref()
    {
        let activity = SongActivity {
            service: SongActivityService::Spotify,
            style: style.into(),
            title: track.name,
            artist: track.artists.iter().map(|artist| artist.name.as_str()).collect::<Vec<&str>>().join(", "),
            album: track.album.name,
            album_cover: track
                .album
                .images
                .first()
                .map_or_else(|| input.user.display_avatar_url("png", None::<String>, 4096), |image| image.url.clone()),
            timestamps: None,
        };

        return ctx.respond(eien("song-card", &[&to_string(&activity)?]).await?, false).await;
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
