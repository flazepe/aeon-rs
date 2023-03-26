use crate::traits::ArgGetters;
use anyhow::Result;
use reqwest::get;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let server = input.get_string_arg("server")?;

    res.send_message(format!(
        "{}'s load is `{}`.",
        server,
        get(format!("https://heliohost.org/load/load_{server}.html").to_lowercase())
            .await?
            .text()
            .await?
            .trim()
    ))
    .await?;

    Ok(())
}
