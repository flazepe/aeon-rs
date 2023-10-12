use crate::structs::{
    api::xe::{statics::XE_CURRENCIES, Xe},
    command::Command,
    command_context::CommandContext,
};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        if ctx.input.is_autocomplete() {
            return ctx.autocomplete(XE_CURRENCIES.iter()).await;
        }

        match Xe::convert(ctx.get_f64_arg("amount")?, ctx.get_string_arg("origin-currency")?, ctx.get_string_arg("target-currency")?).await
        {
            Ok(xe_conversion) => ctx.respond_success(xe_conversion.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
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
