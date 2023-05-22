mod delete;
mod list;
mod select_menu;
mod set;

use crate::structs::command::Command;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| {
    Command::new()
        .subcommand("delete", delete::run)
        .subcommand("list", list::run)
        .subcommand("set", set::run)
        .subcommand("select-menu", select_menu::run)
});

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "reminder",
        description = "Manages your reminders.",
        subcommands = [
			{
                name = "delete",
                description = "Deletes a reminder.",
                options = [
                    {
                        name = "entry",
                        description = "The reminder entry. Can be provided by using the list subcommand",
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
    async fn reminder(mut input: CommandInput, res: CommandResponder) {
        if input.is_string_select() {
            input.subcommand = Some("select-menu".into());
        }

        COMMAND.run(input, res).await?;
    }

    reminder
}
