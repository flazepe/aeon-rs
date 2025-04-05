use crate::{statics::REQWEST, structs::command_context::AeonCommandContext};
use anyhow::{Result, bail};
use nipper::Document;
use std::sync::Arc;

pub async fn run(ctx: Arc<AeonCommandContext>) -> Result<()> {
    let user = ctx.get_string_arg("user")?.to_lowercase();
    let response = REQWEST.get("https://heliohost.org/status/").query(&[("u", &user)]).send().await?;
    let url = response.url().to_string();
    let status = {
        let document = Document::from(&response.text().await?);
        let status = document.select("#page-content p").first().text();
        status.trim().to_string()
    };

    if status.is_empty() || status.contains("no account") {
        bail!("Account not found.")
    }

    ctx.respond(format!("[{user}](<{url}>)\n{status}"), true).await
}
