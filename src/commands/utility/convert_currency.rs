use crate::structs::{
    api::xe::{Xe, statics::XE_CURRENCIES},
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
};
use anyhow::Context;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("convert-currency", &["cc"]).main(|ctx: Arc<AeonCommandContext>| async move {
        if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input {
            if input.is_autocomplete() {
                return ctx.autocomplete(XE_CURRENCIES.iter()).await;
            }
        }

        let (amount, origin_currency, target_currency) = match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(_, _) => {
                (ctx.get_f64_arg("amount")?, ctx.get_string_arg("origin-currency")?, ctx.get_string_arg("target-currency")?)
            },
            AeonCommandInput::MessageCommand(_, args, _) => {
                let mut args = args.split_whitespace();

                (
                    args.next().and_then(|arg| arg.to_lowercase().parse::<f64>().ok()).context("Please provide a valid amount.")?,
                    args.next().map(|arg| arg.to_string()).context("Please provide the origin currency.")?,
                    args.next().map(|arg| arg.to_string()).context("Please provide the target currency.")?,
                )
            },
        };

        let xe = Xe::convert(amount, origin_currency, target_currency).await?;
        ctx.respond_success(xe.format(), false).await
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
