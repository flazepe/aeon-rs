use crate::structs::{api::google::Google, command::Command, command_context::CommandContext, simple_message::SimpleMessage};
use std::sync::LazyLock;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
};

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let message = SimpleMessage::from(ctx.input.target_message.as_ref().unwrap().clone());

        match Google::translate(message, "auto", "en").await {
            Ok(translation) => ctx.respond(translation.format(), true).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "Translate to English",
		command_type = ApplicationCommandType::MESSAGE,
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
	)]
    async fn google_translate_context(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    google_translate_context
}
