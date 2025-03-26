use crate::{
    statics::REQWEST,
    structs::command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Result;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(_, _) = &ctx.command_input else { return Ok(()) };
    let server = ctx.get_string_arg("server")?;

    ctx.respond(
        format!(
            "{server}'s load is `{}`.",
            REQWEST.get(format!("https://heliohost.org/load/load_{server}.html").to_lowercase()).send().await?.text().await?.trim(),
        ),
        true,
    )
    .await
}
