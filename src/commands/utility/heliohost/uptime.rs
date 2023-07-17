use crate::{statics::REQWEST, structs::command_context::CommandContext};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let server = ctx.get_string_arg("server")?;

    ctx.respond(
        format!(
            "{}'s uptime is `{}%`.",
            server,
            REQWEST.get(format!("https://heliohost.org/load/uptime_{server}.html").to_lowercase()).send().await?.text().await?.trim(),
        ),
        false,
    )
    .await
}
