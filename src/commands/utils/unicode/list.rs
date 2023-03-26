use crate::{structs::unicode::UnicodeCharacters, traits::ArgGetters};
use anyhow::Result;
use slashook::commands::{CommandInput, CommandResponder};

pub async fn run(input: CommandInput, res: CommandResponder) -> Result<()> {
    res.send_message(UnicodeCharacters::get(input.get_string_arg("text")?).format())
        .await?;

    Ok(())
}
