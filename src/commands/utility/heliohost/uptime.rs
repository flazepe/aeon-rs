use crate::{
    statics::REQWEST,
    structs::command_context::{AeonCommandContext, CommandInputExt, AeonCommandInput},
};
use anyhow::Result;

pub async fn run(ctx: AeonCommandContext) -> Result<()> {
    let AeonCommandInput::ApplicationCommand(input,  _) = &ctx.command_input else { return Ok(()) };
    let server = input.get_string_arg("server")?;

    ctx.respond(
        format!(
            "{server}'s uptime is `{}%`.",
            REQWEST.get(format!("https://heliohost.org/load/uptime_{server}.html").to_lowercase()).send().await?.text().await?.trim(),
        ),
        true,
    )
    .await
}
