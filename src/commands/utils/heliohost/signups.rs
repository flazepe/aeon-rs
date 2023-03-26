use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(_input: CommandInput, res: CommandResponder) -> Result<()> {
    res.send_message("TODO").await?;

    Ok(())
}
