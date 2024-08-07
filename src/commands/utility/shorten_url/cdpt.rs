use crate::{statics::REQWEST, structs::command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let mut url = ctx.get_string_arg("url")?;

    if !url.starts_with("http") {
        url = format!("http://{url}");
    }

    let shortened_url = REQWEST.get("https://cdpt.in/shorten").query(&[("url", url)]).send().await?.text().await?;
    ctx.respond_success(format!("<{shortened_url}>"), true).await
}
