use crate::{
    structs::{interaction::Interaction, snipes::ReactionSnipes},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };
    let message = input.get_string_arg("message")?;

    match ReactionSnipes::new(input.guild_id.as_ref().unwrap(), message.split("/").last().unwrap()).to_response() {
        Ok(response) => interaction.respond(response, false).await,
        Err(error) => interaction.respond_error(error, true).await,
    }
}
