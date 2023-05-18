use crate::structs::{api::google::Google, command::AeonCommand, command_context::CommandContext, stringified_message::StringifiedMessage};
use anyhow::Result;
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
    async fn google_translate_message(input: CommandInput, res: CommandResponder) {
        AeonCommand::new(input, res).main(run).run().await?;
    }

    google_translate_message
}

async fn run(ctx: CommandContext) -> Result<()> {
    match Google::translate(StringifiedMessage::from(ctx.input.target_message.as_ref().unwrap().clone()), "auto", "en").await {
        Ok(translation) => ctx.respond(translation.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
