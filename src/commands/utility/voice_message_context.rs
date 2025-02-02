use crate::structs::{api::voice_message::VoiceMessage, command::Command, command_context::CommandContext};
use std::sync::LazyLock;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
};

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let audio_url = match ctx.input.target_message.as_ref().unwrap().attachments.first() {
            Some(attachment) => &attachment.url,
            None => return ctx.respond_error("Please provide a media URL or file.", true).await,
        };

        VoiceMessage::send(&ctx.res, audio_url, true).await
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "Send as Voice Message",
        command_type = ApplicationCommandType::MESSAGE,
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
    )]
    async fn voice_message_context(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    voice_message_context
}
