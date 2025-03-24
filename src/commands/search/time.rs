use crate::structs::{
    api::virtualearth::TimeZoneLocation,
    command::Command,
    command_context::{CommandContext, CommandInputExt, Input},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("time", &[]).main(|ctx: CommandContext| async move {
        let location = match &ctx.input {
            Input::ApplicationCommand { input, res: _ } => input.get_string_arg("location")?,
            Input::MessageCommand { message: _, sender: _, args } => args.into(),
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
        COMMAND.run(Input::ApplicationCommand { input, res }).await?;
    }

    func
}
