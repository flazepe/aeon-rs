use crate::{
    statics::REQWEST,
    structs::command_context::{CommandContext, CommandInputExt, Input},
};
use anyhow::Result;
use nipper::Document;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };
    let user = input.get_string_arg("user")?;
    let response = REQWEST.get("https://heliohost.org/status/").query(&[("u", &user)]).send().await?;
    let url = response.url().to_string();
    let status = {
        let document = Document::from(&response.text().await?);
        let status = document.select("#page-content p").first().text();
        status.trim().to_string()
    };

    if status.is_empty() || status.contains("no account") {
        ctx.respond_error("Account not found.", true).await
    } else {
        ctx.respond(format!("[{user}]({url})\n{status}"), true).await
    }
}
