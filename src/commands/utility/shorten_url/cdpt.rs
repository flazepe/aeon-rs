use crate::{statics::REQWEST, structs::command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    ctx.respond_success(
        format!("<{}>", REQWEST.get("https://cdpt.in/shorten").query(&[("url", ctx.get_string_arg("url")?)]).send().await?.text().await?),
        false,
    )
    .await
}
