use crate::structs::{api::steam::Steam, command_context::AeonCommandContext};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let user = Steam::get_user(ctx.get_string_arg("user", 0, true)?).await?;
    ctx.respond(user.format(), false).await
}
