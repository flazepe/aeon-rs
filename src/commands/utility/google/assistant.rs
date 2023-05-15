use crate::{
    structs::{api::google::Google, interaction::Interaction},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    commands::{CommandInput, CommandResponder},
    structs::utils::File,
};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    res.defer(false).await?;

    match Google::query_assistant(input.get_string_arg("query")?).await {
        Ok(image) => interaction.respond(File::new("image.png", image), false).await,
        Err(error) => interaction.respond_error(error, true).await,
    }
}
