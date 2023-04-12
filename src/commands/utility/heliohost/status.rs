use crate::{macros::if_else, statics::emojis::ERROR_EMOJI, traits::ArgGetters};
use anyhow::Result;
use nipper::Document;
use reqwest::get;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let user = input.get_string_arg("user")?;
    let url = format!("https://heliohost.org/status/?u={user}");

    res.send_message({
        let document = Document::from(&get(&url).await?.text().await?);

        let status = document.select("#page-content p").first().text();
        let status = status.trim();

        if_else!(
            status.is_empty() || status.contains("no account"),
            format!("{ERROR_EMOJI} Account not found."),
            format!("[{user}]({url})\n{status}"),
        )
    })
    .await?;

    Ok(())
}
