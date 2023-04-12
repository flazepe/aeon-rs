use crate::structs::{stringified_message::StringifiedMessage, unicode::UnicodeCharacters};
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
        res.send_message(UnicodeCharacters::get(StringifiedMessage::from(input.target_message.unwrap())).format())
            .await?;
    }

    unicode_message
}
