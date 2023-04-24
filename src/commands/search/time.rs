use crate::{
    structs::{api::virtualearth::TimeZoneLocation, interaction::Interaction},
    traits::ArgGetters,
};
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
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        match TimeZoneLocation::get(input.get_string_arg("location")?).await {
            Ok(timezone) => interaction.respond(timezone.format(), false).await?,
            Err(error) => interaction.respond_error(error, true).await?,
        };
    }

    time
}
