use crate::{statics::emojis::*, structs::api::virtualearth::*, traits::*};
use slashook::{command, commands::*, structs::interactions::*};

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
    fn time(input: CommandInput, res: CommandResponder) {
        match TimeZoneLocation::get(input.get_string_arg("location")?).await {
            Ok(timezone) => {
                res.send_message(timezone.format()).await?;
            }
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            }
        };
    }

    time
}
