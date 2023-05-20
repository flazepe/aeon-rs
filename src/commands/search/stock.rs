use crate::structs::{command::AeonCommand, command_context::CommandContext, scraping::stock::Stock};
use anyhow::Result;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

async fn run(ctx: CommandContext) -> Result<()> {
    // We have to defer since scraping this takes a bit of time
    ctx.res.defer(false).await?;

    match Stock::get(ctx.get_string_arg("ticker")?).await {
        Ok(stock) => ctx.respond(stock.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| AeonCommand::new().main(run));

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
        COMMAND.run(input, res).await?;
    }

    stock
}
