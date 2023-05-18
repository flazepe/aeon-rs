use crate::{
    structs::{api::jisho::JishoSearch, command::AeonCommand, command_context::CommandContext, select_menu::SelectMenu},
    traits::ArgGetters,
};
use anyhow::Result;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
		name = "jisho",
		description = "Searches Jisho.",
		options = [
			{
				name = "query",
				description = "The query",
				option_type = InteractionOptionType::STRING,
				required = true,
			},
		],
	)]
    async fn jisho(input: CommandInput, res: CommandResponder) {
        AeonCommand::new(input, res).main(run).run().await?;
    }

    jisho
}

async fn run(ctx: CommandContext) -> Result<()> {
    if ctx.input.is_string_select() {
        return ctx.respond(JishoSearch::get(&ctx.input.values.as_ref().unwrap()[0]).await?.format(), false).await;
    }

    let results = match JishoSearch::search(ctx.input.get_string_arg("query")?).await {
        Ok(results) => results,
        Err(error) => return ctx.respond_error(error, true).await,
    };

    let mut select_menu = SelectMenu::new("jisho", "search", "View other results…", None::<String>);

    for result in &results {
        select_menu = select_menu.add_option(result.format_title(), result.slug.clone(), None::<String>);
    }

    ctx.respond(MessageResponse::from(select_menu).add_embed(results[0].format()), false).await
}
