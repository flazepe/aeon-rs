use crate::{
    statics::REQWEST,
    structs::command_context::{CommandContext, CommandInputExt, Input},
};
use anyhow::Result;

pub async fn run(ctx: CommandContext) -> Result<()> {
    let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };
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
