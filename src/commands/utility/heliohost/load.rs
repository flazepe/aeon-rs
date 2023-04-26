use crate::{statics::REQWEST, structs::interaction::Interaction, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };
    let server = input.get_string_arg("server")?;

    interaction
        .respond(
            format!(
                "{}'s load is `{}`.",
                server,
                REQWEST.get(format!("https://heliohost.org/load/load_{server}.html").to_lowercase()).send().await?.text().await?.trim(),
            ),
            false,
        )
        .await
}
