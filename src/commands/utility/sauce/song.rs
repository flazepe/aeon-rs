use crate::structs::{
    command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
    scraping::anime_song_lyrics::AnimeSongLyrics,
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };

    match AnimeSongLyrics::query(input.get_string_arg("song")?).await {
        Ok(anime_song_lyrics) => ctx.respond(anime_song_lyrics.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
