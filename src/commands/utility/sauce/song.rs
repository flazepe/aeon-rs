use crate::structs::{
    command_context::{CommandContext, CommandInputExt, Input},
    scraping::anime_song_lyrics::AnimeSongLyrics,
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand(input,  _) = &ctx.input else { return Ok(()) };

    match AnimeSongLyrics::query(input.get_string_arg("song")?).await {
        Ok(anime_song_lyrics) => ctx.respond(anime_song_lyrics.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
