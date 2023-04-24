use crate::structs::{api::google::Google, interaction::Interaction, stringified_message::StringifiedMessage};
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
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        match Google::translate(
            StringifiedMessage::from(input.target_message.as_ref().unwrap().clone()),
            "auto",
            "en",
        )
        .await
        {
            Ok(translation) => interaction.respond(translation.format(), false).await?,
            Err(error) => interaction.respond_error(error, true).await?,
        };
    }

    translate_message
}
