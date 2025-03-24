use crate::{
    functions::{hastebin, limit_strings},
    structs::{
        command::Command,
        command_context::{CommandContext, Input},
        simple_message::SimpleMessage,
        unicode::Unicode,
    },
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("List Unicode", &[]).main(|ctx: CommandContext| async move {
        let Input::ApplicationCommand(input, _) = &ctx.input else { return Ok(()) };
        let message = SimpleMessage::from(input.target_message.as_ref().unwrap().clone());
        let mut formatted = Unicode::list(message).format();

        if formatted.len() > 2000 {
            let extra = format!("\n\nFull list: {}", hastebin(&formatted).await?);
            formatted = limit_strings(formatted.split('\n'), '\n', 2000 - extra.len()) + &extra;
        }

        ctx.respond(formatted, true).await
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
        COMMAND.run(Input::ApplicationCommand(input, res)).await?;
    }

    func
}
