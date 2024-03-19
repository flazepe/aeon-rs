use crate::structs::{command::Command, command_context::CommandContext};
use once_cell::sync::Lazy;
use serde_json::{to_string_pretty, Value};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::{
        interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
        utils::File,
        Permissions,
    },
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        let mut response = MessageResponse::from(File::new("message.rs", format!("{:#?}", ctx.input.target_message.as_ref().unwrap())));

        if ctx.input.app_permissions.contains(Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY) {
            if let Ok(value) = ctx
                .input
                .rest
                .get::<Value>(format!(
                    "channels/{}/messages/{}",
                    ctx.input.channel_id.as_ref().unwrap(),
                    ctx.input.target_message.as_ref().unwrap().id,
                ))
                .await
            {
                response = response.add_file(File::new("message.json", to_string_pretty(&value)?))
            }
        }

        ctx.respond(response, true).await
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
