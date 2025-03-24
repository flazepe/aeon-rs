use crate::structs::{
    api::google::Google,
    command::Command,
    command_context::{CommandContext, Input},
    simple_message::SimpleMessage,
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("Translate to English", &[]).main(|ctx: CommandContext| async move {
        let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };
        let message = SimpleMessage::from(input.target_message.as_ref().unwrap().clone());

        match Google::translate(message, "auto", "en").await {
            Ok(translation) => ctx.respond(translation.format(), true).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
		command_type = ApplicationCommandType::MESSAGE,
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
	)]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(Input::ApplicationCommand { input, res }).await?;
    }

    func
}
