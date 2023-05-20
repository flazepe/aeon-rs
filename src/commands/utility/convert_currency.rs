use crate::structs::{
    api::exchange_rate::{statics::EXCHANGE_RATE_CURRENCIES, ExchangeRateConversion},
    command::AeonCommand,
    command_context::CommandContext,
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| {
    AeonCommand::new().main(|ctx: CommandContext| async move {
        if ctx.input.is_autocomplete() {
            return ctx.autocomplete(EXCHANGE_RATE_CURRENCIES.iter()).await;
        }

        match ExchangeRateConversion::get(
            ctx.get_f64_arg("amount")?,
            ctx.get_string_arg("origin-currency")?,
            ctx.get_string_arg("target-currency")?,
        )
        .await
        {
            Ok(exchange_rate_conversion) => ctx.respond_success(exchange_rate_conversion.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

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
        COMMAND.run(input, res).await?;
    }

    convert_currency
}
