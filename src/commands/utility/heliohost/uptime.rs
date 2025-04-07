use crate::{commands::utility::heliohost::HELIOHOST_SERVERS, statics::REQWEST, structs::command_context::AeonCommandContext};
use anyhow::{Context, Result};
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let server = ctx.get_string_arg("server", 0, true)?.to_lowercase();
    let server = HELIOHOST_SERVERS.iter().find(|entry| entry.to_lowercase() == server).context("Invalid server.")?;

    ctx.respond(
        format!(
            "{server}'s uptime is `{}%`.",
            REQWEST.get(format!("https://heliohost.org/load/uptime_{server}.html").to_lowercase()).send().await?.text().await?.trim(),
        ),
        true,
    )
    .await
}
