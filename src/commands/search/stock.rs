use crate::{statics::emojis::*, structs::scraping::stock::*, traits::*};
use slashook::{command, commands::*, structs::interactions::*};

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
    fn stock(input: CommandInput, res: CommandResponder) {
        // We have to defer since scraping this takes a bit of time
        res.defer(false).await?;

        match Stock::get(input.get_string_arg("ticker")?).await {
            Ok(stock) => {
                res.send_message(stock.format()).await?;
            },
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            },
        };
    }

    stock
}
