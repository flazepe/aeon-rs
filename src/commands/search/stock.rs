use crate::structs::{command::AeonCommand, command_context::CommandContext, scraping::stock::Stock};
use anyhow::Result;
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
        AeonCommand::new(input, res).main(run).run().await?;
    }

    stock
}

async fn run(ctx: CommandContext) -> Result<()> {
    // We have to defer since scraping this takes a bit of time
    ctx.res.defer(false).await?;

    match Stock::get(ctx.get_string_arg("ticker")?).await {
        Ok(stock) => ctx.respond(stock.format(), false).await,
        Err(error) => ctx.respond_error(error, true).await,
    }
}
