use crate::{
    statics::emojis::ERROR_EMOJI,
    structs::{api::google::Google, stringified_message::StringifiedMessage},
};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::ApplicationCommandType,
};

pub fn get_command() -> Command {
    #[command(
		name = "Translate to English",
		command_type = ApplicationCommandType::MESSAGE,
	)]
    async fn translate_message(input: CommandInput, res: CommandResponder) {
        match Google::translate(StringifiedMessage::from(input.target_message.unwrap()), "auto", "en").await {
            Ok(translation) => res.send_message(translation.format()).await?,
            Err(error) => res.send_message(format!("{ERROR_EMOJI} {error}")).await?,
        };
    }

    translate_message
}
