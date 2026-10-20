use crate::structs::{command_context::AeonCommandContext, unicode::Unicode};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    ctx.defer(false).await?;

    let unicode = Unicode::list(ctx.get_string_arg("text", 0, true)?, ctx.get_bool_arg("remove-duplicates").unwrap_or_default());
    ctx.respond(unicode.format_pagination("unicode").save().await?, false).await
}
