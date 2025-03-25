use crate::structs::{
    api::jisho::JishoSearch,
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput, CommandInputExt},
    select_menu::SelectMenu,
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("jisho", &["j"]).main(|ctx: AeonCommandContext| async move {
        if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input {
            if input.is_string_select() {
                return ctx.respond(JishoSearch::get(&input.values.as_ref().unwrap()[0]).await?.format(), false).await;
            }
        }

        let query = match &ctx.command_input {
            AeonCommandInput::ApplicationCommand(input, _) => input.get_string_arg("query")?,
            AeonCommandInput::MessageCommand(_, args, _) => args.into(),
        };

        if query.is_empty() {
            return ctx.respond_error("Please provide a query.", true).await;
        }

        let results = match JishoSearch::search(query).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        };

        let select_menu = SelectMenu::new("jisho", "search", "View other results…", Some(&results[0].slug))
            .add_options(results.iter().map(|result| (result.format_title(), result.slug.clone(), None::<String>)));

        ctx.respond(MessageResponse::from(select_menu).add_embed(results[0].format()), false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
		description = "Searches Jisho.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
		options = [
			{
				name = "query",
				description = "The query",
				option_type = InteractionOptionType::STRING,
				required = true,
			},
		],
	)]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
