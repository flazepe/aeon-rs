use crate::structs::{
    api::virtualearth::TimeZoneLocation,
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("time", &[]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        let location = TimeZoneLocation::get(ctx.get_string_arg("location", 0, true)?).await?;
        ctx.respond(location.format(), false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
		description = "Fetches time and date based on the given location.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
		options = [
			{
				name = "location",
				description = "The location",
				option_type = InteractionOptionType::STRING,
				required = true,
			},
		],
	)]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
