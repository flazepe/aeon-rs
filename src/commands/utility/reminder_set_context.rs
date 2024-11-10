use crate::{
    functions::add_reminder_select_options,
    structs::{command::Command, command_context::CommandContext},
};
use std::sync::LazyLock;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        components::{Components, SelectMenu, SelectMenuType},
        interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
    },
};

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new().main({
        |ctx: CommandContext| async move {
            let mut select_menu = SelectMenu::new(SelectMenuType::STRING)
                .set_id(
                    "reminder",
                    format!(
                        "{}/{}/{}",
                        ctx.input.guild_id.as_deref().unwrap_or("@me"),
                        ctx.input.channel_id.as_ref().unwrap(),
                        ctx.input.target_message.as_ref().unwrap().id,
                    ),
                )
                .set_placeholder("Select time to remind about message");

            select_menu = add_reminder_select_options(select_menu);

            ctx.respond(Components::new().add_select_menu(select_menu), true).await
        }
    })
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "Remind me",
        command_type = ApplicationCommandType::MESSAGE,
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
	)]
    async fn reminder_set_context(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    reminder_set_context
}
