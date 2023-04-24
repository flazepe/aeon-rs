use crate::{
    structs::{api::google::Google, interaction::Interaction},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    match Google::query_dns(input.get_string_arg("type")?, input.get_string_arg("domain")?).await {
        Ok(records) => interaction.respond(records.format(), false).await?,
        Err(error) => interaction.respond_error(error, true).await?,
    };

    Ok(())
}
