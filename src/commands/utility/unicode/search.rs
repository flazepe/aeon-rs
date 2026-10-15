use crate::structs::{command_context::AeonCommandContext, unicode::Unicode};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let character = ctx.get_string_arg("character", 0, true)?;
    ctx.defer(false).await?;
    let unicode = Unicode::search(character).await?;
    ctx.respond(unicode.format_pagination("unicode").save().await?, false).await
}
