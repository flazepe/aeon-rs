use crate::{
    functions::{hastebin, limit_strings},
    structs::{command::Command, command_context::CommandContext, simple_message::SimpleMessage, unicode::Unicode},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
};
use std::sync::LazyLock;

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let message = SimpleMessage::from(ctx.input.target_message.as_ref().unwrap().clone());
        let mut formatted = Unicode::list(message).format();

        if formatted.len() > 2000 {
            let extra = format!("\n\nFull list: {}", hastebin(&formatted).await?);
            formatted = limit_strings(formatted.split('\n'), '\n', 2000 - extra.len()) + &extra;
        }

        ctx.respond(formatted, true).await
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "List Unicode",
		command_type = ApplicationCommandType::MESSAGE,
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
	)]
    async fn unicode_list_context(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    unicode_list_context
}
