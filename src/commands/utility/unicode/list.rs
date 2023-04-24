use crate::{
    structs::{interaction::Interaction, unicode::UnicodeCharacters},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    Interaction::new(&input, &res)
        .respond(UnicodeCharacters::get(input.get_string_arg("text")?).format(), false)
        .await
}
