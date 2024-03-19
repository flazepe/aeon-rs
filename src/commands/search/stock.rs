use crate::structs::{command::Command, command_context::CommandContext, scraping::stock::Stock};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| {
        async move {
            // We have to defer since scraping this takes a bit of time
            ctx.res.defer(false).await?;

            match Stock::get(ctx.get_string_arg("ticker")?).await {
                Ok(stock) => ctx.respond(stock.format(), false).await,
                Err(error) => ctx.respond_error(error, true).await,
            }
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "stock",
		description = "Fetches stock information.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
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
        COMMAND.run(input, res).await?;
    }

    stock
}
