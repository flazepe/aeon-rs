use crate::{statics::REQWEST, structs::command_context::CommandContext};
use anyhow::Result;
use nipper::Document;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let user = ctx.get_string_arg("user")?;
    let response = REQWEST.get("https://heliohost.org/status/").query(&[("u", &user)]).send().await?;
    let url = response.url().to_string();

    let status = {
        let document = Document::from(&response.text().await?);
        let status = document.select("#page-content p").first().text();
        status.trim().to_string()
    };

    match status.is_empty() || status.contains("no account") {
        true => ctx.respond_error("Account not found.", true).await,
        false => ctx.respond(format!("[{user}]({url})\n{status}"), true).await,
    }
}
