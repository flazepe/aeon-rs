use crate::structs::{
    api::xe::{Xe, statics::XE_CURRENCIES},
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("convert-currency", &["cc"]).main(|ctx: AeonCommandContext| async move {
        if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input {
            if input.is_autocomplete() {
                return ctx.autocomplete(XE_CURRENCIES.iter()).await;
            }
        }

        let (amount, origin_currency, target_currency) = match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => {
                (input.get_f64_arg("amount")?, input.get_string_arg("origin-currency")?, input.get_string_arg("target-currency")?)
            },
            AeonCommandInput::MessageCommand(_, args, _) => {
                let mut args = args.split_whitespace();

                let Some(amount) = args.next().and_then(|arg| arg.parse::<f64>().ok()) else {
                    return ctx.respond_error("Please provide an amount.", true).await;
                };

                let Some(origin_currency) = args.next().map(|arg| arg.to_string()) else {
                    return ctx.respond_error("Please provide the origin currency.", true).await;
                };

                let Some(target_currency) = args.next().map(|arg| arg.to_string()) else {
                    return ctx.respond_error("Please provide the target currency.", true).await;
                };

                (amount, origin_currency, target_currency)
            },
        };

        match Xe::convert(amount, origin_currency, target_currency).await {
            Ok(xe_conversion) => ctx.respond_success(xe_conversion.format(), false).await,
            Err(error) => ctx.respond_error(error, true).await,
        }
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
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
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
