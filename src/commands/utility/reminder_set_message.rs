use crate::{
    functions::add_reminder_select_options,
    structs::{command::AeonCommand, command_context::CommandContext},
};
use anyhow::Result;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::{
        components::{Components, SelectMenu, SelectMenuType},
        interactions::ApplicationCommandType,
    },
};

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| AeonCommand::new().main(run));

pub fn get_command() -> Command {
    #[command(
		name = "Remind me",
		command_type = ApplicationCommandType::MESSAGE,
	)]
    async fn reminder_set_message(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    reminder_set_message
}

async fn run(ctx: CommandContext) -> Result<()> {
    ctx.respond(
        Components::new().add_select_menu(
            add_reminder_select_options(SelectMenu::new(SelectMenuType::STRING))
                .set_id(
                    "reminder",
                    format!(
                        "{}/{}/{}",
                        ctx.input.guild_id.as_ref().unwrap_or(&"@me".into()),
                        ctx.input.channel_id.as_ref().unwrap(),
                        ctx.input.target_message.as_ref().unwrap().id,
                    ),
                )
                .set_placeholder("Select time to remind about message"),
        ),
        true,
    )
    .await
}
