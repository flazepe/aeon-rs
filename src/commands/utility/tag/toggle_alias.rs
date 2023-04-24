use crate::{
    structs::{interaction::Interaction, tags::Tags},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let interaction = Interaction::new(&input, &res);
    let tag = input.get_string_arg("tag")?;
    let alias = input.get_string_arg("alias")?;

    match Tags::new()
        .toggle_alias(
            &tag,
            input.guild_id.as_ref().unwrap(),
            &alias,
            input.member.as_ref().unwrap(),
        )
        .await
    {
        Ok(response) => interaction.respond_success(response, false).await,
        Err(error) => interaction.respond_error(error, true).await,
    }
}
