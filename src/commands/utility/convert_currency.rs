use crate::structs::{
    api::xe::{statics::XE_CURRENCIES, Xe},
    command::Command,
    command_context::CommandContext,
};
use std::sync::LazyLock;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        if ctx.input.is_autocomplete() {
            return ctx.autocomplete(XE_CURRENCIES.iter()).await;
        }

        let amount = ctx.get_f64_arg("amount")?;
        let origin_currency = ctx.get_string_arg("origin-currency")?;
        let target_currency = ctx.get_string_arg("target-currency")?;

        match Xe::convert(amount, origin_currency, target_currency).await {
            Ok(xe_conversion) => ctx.respond_success(xe_conversion.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "convert-currency",
        description = "Converts a currency to another currency.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
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
