use crate::structs::{api::voice_message::VoiceMessage, command::Command, command_context::CommandContext};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let audio_url =
            match ctx.get_string_arg("media-url").or(ctx.get_attachment_arg("media-file").map(|attachment| attachment.url.clone())) {
                Ok(url) => url,
                Err(_) => return ctx.respond_error("Please provide a media URL or file.", true).await,
            };

        VoiceMessage::send(&ctx.res, audio_url, false).await
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "voice-message",
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
    async fn voice_message(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    voice_message
}
