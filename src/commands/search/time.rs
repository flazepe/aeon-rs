use crate::{
    structs::{api::virtualearth::TimeZoneLocation, command::AeonCommand, command_context::CommandContext},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

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
        AeonCommand::new(input, res).main(run).run().await?;
    }

    time
}

async fn run(ctx: CommandContext) -> Result<()> {
    match TimeZoneLocation::get(ctx.input.get_string_arg("location")?).await {
        Ok(timezone) => ctx.respond(timezone.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
