use crate::structs::{interaction::Interaction, stringified_message::StringifiedMessage, unicode::UnicodeCharacters};
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
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        interaction
            .respond(
                UnicodeCharacters::get(StringifiedMessage::from(input.target_message.as_ref().unwrap().clone()))
                    .format(),
                false,
            )
            .await?;
    }

    unicode_message
}
