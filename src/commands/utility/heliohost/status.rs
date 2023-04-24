use crate::{macros::if_else, structs::interaction::Interaction, traits::ArgGetters};
use anyhow::Result;
use nipper::Document;
use reqwest::Client;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let user = input.get_string_arg("user")?;

    let response = Client::new()
        .get("https://heliohost.org/status/")
        .query(&[("u", user.as_str())])
        .send()
        .await?;

    let url = response.url().to_string();

    let status = {
        let document = Document::from(&response.text().await?);
        let status = document.select("#page-content p").first().text();
        status.trim().to_string()
    };

    let interaction = Interaction::new(&input, &res);

    if_else!(
        status.is_empty() || status.contains("no account"),
        interaction.respond_error("Account not found.", true).await,
        interaction.respond(format!("[{user}]({url})\n{status}"), false).await,
    )
}
