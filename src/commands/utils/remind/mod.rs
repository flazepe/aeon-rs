mod delete;
mod list;
mod set;
mod snooze;

use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

pub fn get_command() -> Command {
    #[command(
        name = "remind",
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
    async fn remind(input: CommandInput, res: CommandResponder) {
        // Snooze
        if input.custom_id == Some("time".into()) {
            return snooze::run(input, res).await?;
        }

        match input.subcommand.as_deref().unwrap_or("") {
            "delete" => delete::run(input, res).await?,
            "list" => list::run(input, res).await?,
            "set" => set::run(input, res, false).await?,
            _ => {},
        }
    }

    remind
}
