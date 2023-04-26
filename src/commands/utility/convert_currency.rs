use crate::{
    structs::{
        api::exchange_rate::{statics::EXCHANGE_RATE_CURRENCIES, ExchangeRateConversion},
        interaction::Interaction,
    },
    traits::ArgGetters,
};
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
        name = "convert-currency",
        description = "Converts a currency to another currency.",
        options = [
            {
                name = "amount",
                description = "The amount of currency",
                option_type = InteractionOptionType::NUMBER,
                required = true,
            },
            {
                name = "origin-currency",
                description = "The origin currency, e.g. GBP, NOK, USD",
                option_type = InteractionOptionType::STRING,
                autocomplete = true,
                required = true,
            },
            {
                name = "target-currency",
                description = "The currency to convert the amount to, e.g. GBP, NOK, USD",
                option_type = InteractionOptionType::STRING,
                autocomplete = true,
                required = true,
            },
        ],
    )]
    async fn convert_currency(input: CommandInput, res: CommandResponder) {
        let Ok(interaction) = Interaction::new(&input, &res).verify().await else { return Ok(()); };

        if input.is_autocomplete() {
            return interaction.hashmap_autocomplete(EXCHANGE_RATE_CURRENCIES.iter()).await?;
        }

        match ExchangeRateConversion::get(
            input.get_f64_arg("amount")?,
            input.get_string_arg("origin-currency")?,
            input.get_string_arg("target-currency")?,
        )
        .await
        {
            Ok(exchange_rate_conversion) => interaction.respond_success(exchange_rate_conversion.format(), false).await?,
            Err(error) => interaction.respond_error(error, true).await?,
        };
    }

    convert_currency
}
