use crate::structs::{
    api::voice_message::VoiceMessage,
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Context;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("voice-message", &[]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let AeonCommandInput::ApplicationCommand(_, res) = &ctx.command_input else { return Ok(()) };

        let audio_url = ctx
            .get_string_arg("media-url")
            .or(ctx.get_attachment_arg("media-file").map(|attachment| attachment.url.clone()))
            .context("Please provide a media URL or file.")?;

        VoiceMessage::send(res, audio_url, false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
		description = "Sends a media file as a voice message.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
		options = [
			{
				name = "media-url",
				description = "The media URL",
				option_type = InteractionOptionType::STRING,
			},
			{
				name = "media-file",
				description = "The media file",
				option_type = InteractionOptionType::ATTACHMENT,
			},
        ]
	)]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
