mod delete;
mod list;
mod select_menu;
mod set;

use crate::structs::{command::AeonCommand, command_context::AeonCommandInput};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("reminder", &[])
        .subcommand("delete", &[], delete::run)
        .subcommand("list", &["ls"], list::run)
        .subcommand("set", &[], set::run)
        .subcommand("select-menu", &[], select_menu::run)
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Manages your reminders.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        subcommands = [
			{
                name = "delete",
                description = "Deletes a reminder.",
                options = [
                    {
                        name = "reminder",
                        description = "The reminder. Can be retrieved using the list subcommand",
                        option_type = InteractionOptionType::STRING,
                        autocomplete = true,
                        required = true,
                    },
                ],
            },
            {
                name = "list",
                description = "Sends the reminder list.",
            },
            {
                name = "set",
                description = "Sets a reminder.",
                options = [
                    {
                        name = "time",
                        description = "The duration to remind, e.g. 1h",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
                    {
                        name = "reminder",
                        description = "The reminder",
                        option_type = InteractionOptionType::STRING,
						max_length = 200,
                    },
                    {
                        name = "interval",
                        description = "The interval time to remind, e.g. 1h",
                        option_type = InteractionOptionType::STRING
                    },
					{
                        name = "dm",
                        description = "Whether to DM instead",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
                ],
            },
        ],
    )]
    async fn func(mut input: CommandInput, res: CommandResponder) {
        if input.is_string_select() {
            input.subcommand = Some("select-menu".into());
        }

        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
