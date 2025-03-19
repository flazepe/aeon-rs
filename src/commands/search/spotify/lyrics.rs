use crate::{
    statics::CACHE,
    structs::{api::spotify::Spotify, command_context::CommandContext, select_menu::SelectMenu},
};
use anyhow::Result;
use slashook::commands::MessageResponse;

pub async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.is_string_select() {
        ctx.defer(false).await?;

        let lyrics = match Spotify::get_track(&ctx.input.values.as_ref().unwrap()[0]).await {
            Ok(track) => Spotify::get_lyrics(track).await,
            Err(error) => return ctx.respond_error(error, true).await,
        };

        return match lyrics {
            Ok(lyrics) => ctx.respond(lyrics.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        };
    }

    let Some(query) = ctx
        .get_string_arg("song")
        .ok()
        .or_else(|| CACHE.song_activities.read().unwrap().get(&ctx.input.user.id).map(|song| format!("{} - {}", song.artist, song.title)))
    else {
        return ctx.respond_error("Please provide a song.", true).await;
    };

    ctx.defer(false).await?;

    let mut select_menu = SelectMenu::new("spotify", "lyrics", "View other results…", None::<String>);

    let mut tracks = match Spotify::search_track(query).await {
        Ok(tracks) => tracks,
        Err(error) => return ctx.respond_error(error, true).await,
    };

    for track in &tracks {
        select_menu = select_menu.add_option(&track.name, &track.id, Some(&track.artists[0].name));
    }

    match Spotify::get_lyrics(tracks.remove(0)).await {
        Ok(lyrics) => ctx.respond(MessageResponse::from(select_menu).add_embed(lyrics.format()), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
