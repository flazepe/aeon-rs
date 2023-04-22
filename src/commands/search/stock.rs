use crate::{statics::emojis::ERROR_EMOJI, structs::scraping::stock::Stock, traits::ArgGetters};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
		name = "stock",
		description = "Fetches stock information.",
		options = [
			{
				name = "ticker",
				description = "The ticker",
				option_type = InteractionOptionType::STRING,
				required = true,
			},
		],
	)]
    async fn stock(input: CommandInput, res: CommandResponder) {
        // We have to defer since scraping this takes a bit of time
        res.defer(false).await?;

        match Stock::get(input.get_string_arg("ticker")?).await {
            Ok(stock) => {
                res.send_message(stock.format()).await?;
            },
            Err(error) => {
                res.send_message(MessageResponse::from(format!("{ERROR_EMOJI} {error}")).set_ephemeral(true))
                    .await?;
            },
        };
    }

    stock
}
