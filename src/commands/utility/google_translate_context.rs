use crate::structs::{
    api::google::Google,
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
    simple_message::SimpleMessage,
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("Translate to English", &[]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
        let message = SimpleMessage::from(input.target_message.as_ref().unwrap().clone());
        let translation = Google::translate(message, "auto", "en").await?;

        ctx.respond(translation.format(), true).await
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
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
