use crate::{
    structs::{interaction::Interaction, scraping::stock::Stock},
    traits::ArgGetters,
};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
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
        let interaction = Interaction::new(&input, &res);

        // We have to defer since scraping this takes a bit of time
        res.defer(false).await?;

        match Stock::get(input.get_string_arg("ticker")?).await {
            Ok(stock) => interaction.respond(stock.format(), false).await?,
            Err(error) => interaction.respond_error(error, true).await?,
        };
    }

    stock
}
