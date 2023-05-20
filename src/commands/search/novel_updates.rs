use crate::structs::{api::localdown::LocalDownNovel, command::AeonCommand, command_context::CommandContext, select_menu::SelectMenu};
use anyhow::Result;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder, MessageResponse},
    structs::interactions::InteractionOptionType,
};

async fn run(ctx: CommandContext) -> Result<()> {
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

    let mut select_menu = SelectMenu::new("novel-updates", "novel-updates", "Select a novel…", None::<String>);

    for result in &results {
        select_menu = select_menu.add_option(&result.title, result.id, None::<String>);
    }

    ctx.respond(
        MessageResponse::from(select_menu).add_embed(match LocalDownNovel::get(results[0].id).await {
            Ok(result) => result.format(),
            Err(error) => return ctx.respond_error(error, true).await,
        }),
        false,
    )
    .await
}

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| AeonCommand::new().main(run));

pub fn get_command() -> Command {
    #[command(
		name = "novel-updates",
		description = "Fetches a novel from Novel Updates.",
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
