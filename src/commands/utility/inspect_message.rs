use crate::structs::{
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use serde_json::{Value, to_string_pretty};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::{
        Permissions,
        interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
        utils::File,
    },
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("Inspect Message", &[]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };
        let mut response = MessageResponse::from(File::new("message.rs", format!("{:#?}", input.target_message.as_ref().unwrap())));

        if input.app_permissions.contains(Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY) {
            if let Ok(value) = input
                .rest
                .get::<Value>(format!(
                    "channels/{}/messages/{}",
                    input.channel_id.as_ref().unwrap(),
                    input.target_message.as_ref().unwrap().id.as_deref().unwrap_or_default(),
                ))
                .await
            {
                response = response.add_file(File::new("message.json", to_string_pretty(&value)?))
            }
        }

        ctx.respond(response, true).await
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
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
