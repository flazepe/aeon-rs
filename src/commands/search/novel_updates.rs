use crate::structs::{
    api::localdown::LocalDownNovel,
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
    AeonCommand::new("novel-updates", &["nu"]).main(|ctx: AeonCommandContext| async move {
        let AeonCommandInput::ApplicationCommand(input, _) = &ctx.command_input else { return Ok(()) };

        if input.is_string_select() {
            return match LocalDownNovel::get(input.values.as_ref().unwrap()[0].parse::<u64>()?).await {
                Ok(result) => ctx.respond(result.format(), false).await,
                Err(error) => ctx.respond_error(error, true).await,
            };
        }

        let results = match LocalDownNovel::search(input.get_string_arg("novel")?).await {
            Ok(results) => results,
            Err(error) => return ctx.respond_error(error, true).await,
        };

        let select_menu = SelectMenu::new("novel-updates", "novel-updates", "View other novels…", Some(results[0].id))
            .add_options(results.iter().map(|result| (&result.title, result.id, None::<String>)));

        let embed = match LocalDownNovel::get(results[0].id).await {
            Ok(novel) => novel.format(),
            Err(error) => return ctx.respond_error(error, true).await,
        };

        ctx.respond(MessageResponse::from(select_menu).add_embed(embed), false).await
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
