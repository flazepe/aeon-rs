use crate::structs::{api::virtualearth::TimeZoneLocation, command::Command, command_context::CommandContext};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        match TimeZoneLocation::get(ctx.get_string_arg("location")?).await {
            Ok(timezone) => ctx.respond(timezone.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "time",
		description = "Fetches time and date based on the given location.",
		options = [
			{
				name = "location",
				description = "The location",
				option_type = InteractionOptionType::STRING,
				required = true,
			},
		],
	)]
    async fn time(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    time
}
