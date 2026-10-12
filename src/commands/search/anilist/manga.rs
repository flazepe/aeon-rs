use crate::structs::{api::anilist::AniList, command_context::AeonCommandContext};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let manga = AniList::search_manga(ctx.get_string_arg("manga", 0, true)?).await?.remove(0);

    if manga.is_adult {
        ctx.ensure_nsfw_channel().await?;
    }

    ctx.respond_pagination(manga.format_pagination("anilist"), false).await
}
