use crate::structs::{api::voice_message::VoiceMessage, command::Command, command_context::CommandContext};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        VoiceMessage::send(
            &ctx.res,
            match ctx.input.target_message.as_ref().unwrap().attachments.first() {
                Some(attachment) => &attachment.url,
                None => return ctx.respond_error("Please provide an audio URL or file.", true).await,
            },
            true,
        )
        .await
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
