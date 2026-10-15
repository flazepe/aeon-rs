use crate::structs::{api::anilist::AniList, command_context::AeonCommandContext};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let anime = AniList::search_anime(ctx.get_string_arg("anime", 0, true)?).await?.remove(0);

    if anime.is_adult {
        ctx.ensure_nsfw_channel().await?;
    }

    ctx.respond(anime.format_pagination("anilist").save().await?, false).await
}
