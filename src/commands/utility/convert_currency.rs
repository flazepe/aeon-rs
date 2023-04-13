use crate::{
    functions::hashmap_autocomplete,
    statics::{emojis::ERROR_EMOJI, exchange_rate::EXCHANGE_RATE_CURRENCIES},
    structs::api::exchange_rate::ExchangeRateConversion,
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
        if input.is_autocomplete() {
            return hashmap_autocomplete(input, res, EXCHANGE_RATE_CURRENCIES.iter()).await?;
        }

        match ExchangeRateConversion::get(
            input.get_f64_arg("amount")?,
            input.get_string_arg("origin-currency")?,
            input.get_string_arg("target-currency")?,
        )
        .await
        {
            Ok(exchange_rate_conversion) => {
                res.send_message(exchange_rate_conversion.format()).await?;
            },
            Err(error) => {
                res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
            },
        };
    }

    convert_currency
}
