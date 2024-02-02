use crate::structs::{api::voice_message::VoiceMessage, command::Command, command_context::CommandContext};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        ctx.res.defer(true).await?;

        match VoiceMessage::send(
            ctx.input.channel_id.as_ref().unwrap_or(&"".into()),
            None::<String>,
            match ctx.get_string_arg("audio-url").or(ctx.get_attachment_arg("audio-file").map(|attachment| attachment.url.clone())) {
                Ok(url) => url,
                Err(_) => return ctx.respond_error("Please provide an audio URL or file.", true).await,
            },
        )
        .await
        {
            Ok(_) => ctx.respond_success("Sent.", true).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "voice-message",
		description = "Sends an audio file as a voice message.",
		options = [
			{
				name = "audio-url",
				description = "The audio URL",
				option_type = InteractionOptionType::STRING,
			},
			{
				name = "audio-file",
				description = "The audio file",
				option_type = InteractionOptionType::ATTACHMENT,
			},
        ]
	)]
    async fn voice_message(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    voice_message
}
