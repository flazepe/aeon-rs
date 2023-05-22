use crate::structs::{api::google::Google, command::Command, command_context::CommandContext, stringified_message::StringifiedMessage};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::ApplicationCommandType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        match Google::translate(StringifiedMessage::from(ctx.input.target_message.as_ref().unwrap().clone()), "auto", "en").await {
            Ok(translation) => ctx.respond(translation.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "Translate to English",
		command_type = ApplicationCommandType::MESSAGE,
	)]
    async fn google_translate_message(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    google_translate_message
}
