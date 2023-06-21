use crate::{structs::command_context::CommandContext, structs::scraping::anime_song_lyrics::AnimeSongLyrics};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    match AnimeSongLyrics::query(ctx.get_string_arg("song")?).await {
        Ok(anime_song_lyrics) => ctx.respond(anime_song_lyrics.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
