use crate::{structs::interaction::Interaction, traits::ArgGetters};
use anyhow::Result;
use reqwest::get;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };
    let server = input.get_string_arg("server")?;

    interaction
        .respond(
            format!(
                "{}'s uptime is `{}`.",
                server,
                get(format!("https://heliohost.org/load/uptime_{server}.html").to_lowercase()).await?.text().await?.trim(),
            ),
            false,
        )
        .await
}
