use crate::{statics::emojis::*, structs::api::google_translate::*, *};
use slashook::{command, commands::*, structs::interactions::*};

pub fn get_command() -> Command {
    #[command(
		name = "Translate to English",
		command_type = ApplicationCommandType::MESSAGE,
	)]
    async fn translate_message(input: CommandInput, res: CommandResponder) {
        match GoogleTranslate::translate(
            stringify_message!(input.target_message.as_ref().unwrap(), vec![]),
            "auto",
            "en",
        )
        .await
        {
            Ok(translation) => {
                res.send_message(translation.format()).await?;
            },
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            },
        }
    }

    translate_message
}
