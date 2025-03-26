use crate::structs::{command_context::AeonCommandContext, scraping::anime_song_lyrics::AnimeSongLyrics};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let anime_song_lyrics = AnimeSongLyrics::query(ctx.get_string_arg("song")?).await?;
    ctx.respond(anime_song_lyrics.format(), false).await
}
