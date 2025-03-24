use crate::{
    functions::add_reminder_select_options,
    structs::{
        command::Command,
        command_context::{CommandContext, Input},
    },
};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        components::{Components, SelectMenu, SelectMenuType},
        interactions::{ApplicationCommandType, IntegrationType, InteractionContextType},
    },
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new("Remind me", &[]).main({
        |ctx: CommandContext| async move {
            let Input::ApplicationCommand { input, res: _ } = &ctx.input else { return Ok(()) };

            let mut select_menu = SelectMenu::new(SelectMenuType::STRING)
                .set_id(
                    "reminder",
                    format!(
                        "{}/{}/{}",
                        input.guild_id.as_deref().unwrap_or("@me"),
                        input.channel_id.as_ref().unwrap(),
                        input.target_message.as_ref().unwrap().id,
                    ),
                )
                .set_placeholder("Select time to remind about message");

            select_menu = add_reminder_select_options(select_menu);

            ctx.respond(Components::new().add_select_menu(select_menu), true).await
        }
    })
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        command_type = ApplicationCommandType::MESSAGE,
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
	)]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(Input::ApplicationCommand { input, res }).await?;
    }

    func
}
