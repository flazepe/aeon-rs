use crate::{statics::MONGODB, structs::command_context::AeonCommandContext};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let name = ctx.get_string_arg("name", 0, true)?;

    let mongodb = MONGODB.get().unwrap();
    mongodb.code_snippets.delete(name, ctx.get_user_id()).await?;

    ctx.respond_success("Gone.", true).await
}
