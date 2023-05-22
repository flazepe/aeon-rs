use crate::structs::{api::jisho::JishoSearch, command::Command, command_context::CommandContext, select_menu::SelectMenu};
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        if ctx.input.is_string_select() {
            return ctx.respond(JishoSearch::get(&ctx.input.values.as_ref().unwrap()[0]).await?.format(), false).await;
        }

        let results = match JishoSearch::search(ctx.get_string_arg("query")?).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        };

        let mut select_menu = SelectMenu::new("jisho", "search", "View other results…", None::<String>);

        for result in &results {
            select_menu = select_menu.add_option(result.format_title(), result.slug.clone(), None::<String>);
        }

        ctx.respond(MessageResponse::from(select_menu).add_embed(results[0].format()), false).await
    })
});

pub fn get_command() -> SlashookCommand {
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
        COMMAND.run(input, res).await?;
    }

    jisho
}
