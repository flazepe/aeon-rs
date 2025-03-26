use crate::{
    statics::CACHE,
    structs::{
        api::{google::statics::GOOGLE_TRANSLATE_LANGUAGES, spotify::Spotify},
        command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
        select_menu::SelectMenu,
    },
};
use anyhow::{Result, bail};
use slashook::commands::MessageResponse;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input {
        if input.is_autocomplete() {
            return ctx.autocomplete(GOOGLE_TRANSLATE_LANGUAGES.iter()).await;
        }

        if input.is_string_select() {
            ctx.defer(false).await?;

            let track = Spotify::get_track(&input.values.as_ref().unwrap()[0]).await?;
            let lyrics = Spotify::get_lyrics(track, None::<String>).await?;

            return ctx.respond(lyrics.format(), false).await;
        }
    }

    let (user_id, query, translate_language) = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, _) => {
            (input.user.id.clone(), input.get_string_arg("song").ok(), input.get_string_arg("translate").ok())
        },
        AeonCommandInput::MessageCommand(message, args, _) => {
            (message.author.id.to_string(), if args.trim().is_empty() { None } else { Some(args.into()) }, None::<String>)
        },
    };

    let query = query
        .or_else(|| CACHE.song_activities.read().unwrap().get(&user_id).map(|song| format!("{} - {}", song.artist, song.title)))
        .unwrap_or_default();

    if query.is_empty() {
        bail!("Please provide a song.");
    }

    ctx.defer(false).await?;

    let mut tracks = Spotify::search_track(query).await?;
    let select_menu = SelectMenu::new("spotify", "lyrics", "View other lyrics…", Some(&tracks[0].id))
        .add_options(tracks.iter().map(|track| (&track.name, &track.id, Some(&track.artists[0].name))));
    let lyrics = Spotify::get_lyrics(tracks.remove(0), translate_language).await?;

    ctx.respond(MessageResponse::from(select_menu).add_embed(lyrics.format()), false).await
}
