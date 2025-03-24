use crate::structs::{
    api::virtualearth::TimeZoneLocation,
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("time", &[]).main(|ctx: AeonCommandContext| async move {
        let location = match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input.get_string_arg("location")?,
            AeonCommandInput::MessageCommand(_, args, _) => args.into(),
        };

        if location.is_empty() {
            return ctx.respond_error("Please provide a location.", true).await;
        }

        match TimeZoneLocation::get(location).await {
            Ok(timezone) => ctx.respond(timezone.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
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
