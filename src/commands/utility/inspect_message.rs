use crate::structs::{command::Command, command_context::CommandContext};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::{
        interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
        utils::File,
    },
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        ctx.respond(MessageResponse::from(File::new("message.rs", format!("{:#?}", ctx.input.target_message.as_ref().unwrap()))), true)
            .await
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "Inspect Message",
		command_type = ApplicationCommandType::MESSAGE,
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
	)]
    async fn inspect_message(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    inspect_message
}
