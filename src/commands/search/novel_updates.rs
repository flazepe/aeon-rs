use crate::structs::{api::localdown::LocalDownNovel, command::Command, command_context::CommandContext, select_menu::SelectMenu};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main(|ctx: CommandContext| async move {
        if ctx.input.is_string_select() {
            return match LocalDownNovel::get(ctx.input.values.as_ref().unwrap()[0].parse::<u64>()?).await {
                Ok(result) => ctx.respond(result.format(), false).await,
                Err(error) => ctx.respond_error(error, true).await,
            };
        }

        let results = match LocalDownNovel::search(ctx.get_string_arg("novel")?).await {
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

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "novel-updates",
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
    async fn localdown(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    localdown
}
