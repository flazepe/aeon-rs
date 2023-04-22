use crate::{macros::if_else, statics::emojis::ERROR_EMOJI, traits::ArgGetters};
use anyhow::Result;
use nipper::Document;
use reqwest::Client;
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let user = input.get_string_arg("user")?;

    let response = Client::new()
        .get("https://heliohost.org/status/")
        .query(&[("u", user.as_str())])
        .send()
        .await?;

    let url = response.url().to_string();

    res.send_message({
        let document = Document::from(&response.text().await?);
        let status = document.select("#page-content p").first().text();
        let status = status.trim();

        if_else!(
            status.is_empty() || status.contains("no account"),
            MessageResponse::from(format!("{ERROR_EMOJI} Account not found.")).set_ephemeral(true),
            MessageResponse::from(format!("[{user}]({url})\n{status}")),
        )
    })
    .await?;

    Ok(())
}
