use crate::{
    structs::{interaction::Interaction, tags::Tags},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    match Tags::new().delete(input.get_string_arg("tag")?, input.guild_id.as_ref().unwrap(), input.member.as_ref().unwrap()).await {
        Ok(response) => interaction.respond_success(response, true).await,
        Err(error) => interaction.respond_error(error, true).await,
    }
}
