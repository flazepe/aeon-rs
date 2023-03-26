use crate::{macros::stringify_message, structs::unicode::UnicodeCharacters};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::ApplicationCommandType,
};

pub fn get_command() -> Command {
    #[command(
        name = "List Unicodes",
        command_type = ApplicationCommandType::MESSAGE,
    )]
    async fn unicode_message(input: CommandInput, res: CommandResponder) {
        res.send_message(
            UnicodeCharacters::get(stringify_message!(input.target_message.as_ref().unwrap(), vec![])).format(),
        )
        .await?;
    }

    unicode_message
}
