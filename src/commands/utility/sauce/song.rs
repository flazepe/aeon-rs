use crate::structs::{
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    scraping::anime_song_lyrics::AnimeSongLyrics,
};
use anyhow::{Result, bail};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let song = match &ctx.command_input {
        AeonCommandInput::ApplicationCommand(input, _) => input.get_string_arg("song")?,
        AeonCommandInput::MessageCommand(_, args, _) => args.into(),
    };

    if song.is_empty() {
        bail!("Please provide a song.");
    }

    let anime_song_lyrics = AnimeSongLyrics::query(song).await?;
    ctx.respond(anime_song_lyrics.format(), false).await
}
