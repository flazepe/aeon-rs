use crate::structs::{
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
    simple_message::SimpleMessage,
    unicode::Unicode,
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("List Unicode", &[]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
        let message = SimpleMessage::from(input.target_message.as_ref().unwrap().clone());
        let unicode = Unicode::list(message);

        ctx.respond(unicode.format_pagination("List Unicode").save().await?, true).await
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
        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
