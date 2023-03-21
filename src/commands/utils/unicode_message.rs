use crate::{stringify_message, structs::unicode::*};
use anyhow::Context;
use slashook::{command, commands::*, structs::interactions::*};

pub fn get_command() -> Command {
    #[command(
        name = "List Unicodes",
        command_type = ApplicationCommandType::MESSAGE,
    )]
    async fn unicode_message(input: CommandInput, res: CommandResponder) {
        res.send_message(
            UnicodeCharacters::get(stringify_message!(
                input
                    .target_message
                    .as_ref()
                    .context("missing target message")?,
                vec![]
            ))
            .format(),
        )
        .await?;
    }

    unicode_message
}
