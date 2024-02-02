use crate::structs::{
    command::Command, command_context::CommandContext, stringified_message::StringifiedMessage, unicode::UnicodeCharacters,
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::ApplicationCommandType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        ctx.respond(UnicodeCharacters::get(StringifiedMessage::from(ctx.input.target_message.as_ref().unwrap().clone())).format(), false)
            .await
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "List Unicodes",
        command_type = ApplicationCommandType::MESSAGE,
    )]
    async fn unicode_context(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    unicode_context
}
