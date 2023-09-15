use crate::structs::command_context::CommandContext;
use anyhow::Result;
use slashook::structs::utils::File;
use tokio::process::Command;

pub async fn run(ctx: CommandContext) -> Result<()> {
    ctx.res.defer(false).await?;

    ctx.respond(
        File::new(
            "image.png",
            Command::new("node").args(["../eien", &ctx.get_string_arg("command")?, &ctx.get_string_arg("args")?]).output().await?.stdout,
        ),
        false,
    )
    .await
}
