use crate::{
    structs::{interaction::Interaction, unicode::UnicodeCharacter},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

    match UnicodeCharacter::get(input.get_string_arg("query")?).await {
        Ok(unicode_character) => interaction.respond(unicode_character.format(), false).await,
        Err(error) => interaction.respond_error(error, true).await,
    }
}
