use crate::structs::{command_context::AeonCommandContext, unicode::Unicode};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let list = Unicode::list(ctx.get_string_arg("text", 0, true)?);
    ctx.respond(list.format_pagination("unicode").save().await?, false).await
}
