use crate::structs::{api::anilist::AniList, command_context::AeonCommandContext};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let user = AniList::get_user(ctx.get_string_arg("user", 0, true)?).await?;
    ctx.respond_pagination(user.format_pagination("anilist"), false).await
}
