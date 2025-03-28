use crate::structs::{
    api::jisho::JishoSearch,
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
    select_menu::SelectMenu,
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("jisho", &["j"]).main(|ctx: Arc<AeonCommandContext>| async move {
        if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input {
            if input.is_string_select() {
                let jisho = JishoSearch::get(&input.values.as_ref().unwrap()[0]).await?;
                return ctx.respond(jisho.format(), false).await;
            }
        }

        let results = JishoSearch::search(ctx.get_string_arg("query")?).await?;

        let select_menu = SelectMenu::new("jisho", "search", "View other results…", None::<String>)
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
