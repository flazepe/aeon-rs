use crate::structs::{api::voice_message::VoiceMessage, command::Command, command_context::CommandContext};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::ApplicationCommandType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        ctx.res.defer(true).await?;

        let message = ctx.input.target_message.as_ref().unwrap();

        match VoiceMessage::send(
            ctx.input.channel_id.as_ref().unwrap_or(&"".into()),
            Some(&message.id),
            match message.attachments.get(0) {
                Some(attachment) => &attachment.url,
                None => return ctx.respond_error("Please provide an audio URL or file.", true).await,
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
        name = "Send as Voice Message",
        command_type = ApplicationCommandType::MESSAGE,
    )]
    async fn voice_message_context(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    voice_message_context
}
