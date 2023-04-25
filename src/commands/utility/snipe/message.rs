use crate::{
    structs::{interaction::Interaction, snipes::Snipes},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    match Snipes::new(
        input.get_channel_arg("channel").map_or(input.channel_id.as_ref().unwrap(), |channel| &channel.id),
        input.get_bool_arg("edit").unwrap_or(false),
        input.get_bool_arg("list").unwrap_or(false),
    )
    .to_response()
    {
        Ok(response) => interaction.respond(response, false).await,
        Err(error) => interaction.respond_error(error, true).await,
    }
}
