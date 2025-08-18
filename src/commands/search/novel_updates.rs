use crate::structs::{
    command::AeonCommand,
    command_context::{AeonCommandContext, AeonCommandInput},
    scraping::novel_updates::NovelUpdates,
    select_menu::SelectMenu,
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::{Arc, LazyLock};

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("novel-updates", &["novel", "nu"]).set_main(|ctx: Arc<AeonCommandContext>| async move {
        ctx.defer(false).await?;

        if let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input
            && input.is_string_select()
        {
            let novel = NovelUpdates::get(&input.values.as_ref().unwrap()[0]).await?;
            return ctx.respond(novel.format(), false).await;
        }

        let results = NovelUpdates::search(ctx.get_string_arg("novel", 0, true)?).await?;
        let options = results.iter().map(|result| (&result.title, &result.id, None::<String>));
        let select_menu = SelectMenu::new("novel-updates", "novel-updates", "View other novels…", None::<String>).add_options(options);
        let novel = NovelUpdates::get(&results[0].id).await?;

        ctx.respond(MessageResponse::from(select_menu).add_embed(novel.format()), false).await
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
		description = "Fetches a novel from Novel Updates.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
		options = [
			{
				name = "novel",
				description = "The novel",
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
