use crate::structs::{api::virtualearth::TimeZoneLocation, command::AeonCommand, command_context::CommandContext};
use anyhow::Result;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

async fn run(ctx: CommandContext) -> Result<()> {
    match TimeZoneLocation::get(ctx.get_string_arg("location")?).await {
        Ok(timezone) => ctx.respond(timezone.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| AeonCommand::new().main(run));

pub fn get_command() -> Command {
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
