use crate::{statics::emojis::ERROR_EMOJI, structs::api::virtualearth::TimeZoneLocation, traits::ArgGetters};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, MessageResponse},
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
        match TimeZoneLocation::get(input.get_string_arg("location")?).await {
            Ok(timezone) => {
                res.send_message(timezone.format()).await?;
            },
            Err(error) => {
                res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} {error}")).set_ephemeral(true))
                    .await?;
            },
        };
    }

    time
}
