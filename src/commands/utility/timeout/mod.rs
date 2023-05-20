mod remove;
mod set;

use crate::structs::command::AeonCommand;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::{interactions::InteractionOptionType, Permissions},
};

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| AeonCommand::new().subcommand("remove", remove::run).subcommand("set", set::run));

pub fn get_command() -> Command {
    #[command(
		name = "timeout",
		description = "Manages members' timeout.",
		default_member_permissions = Permissions::MODERATE_MEMBERS,
		dm_permission = false,
		subcommands = [
			{
				name = "remove",
				description = "Removes a member's timeout.",
				options = [
					{
						name = "member",
						description = "The member",
						option_type = InteractionOptionType::USER,
						required = true,
					},
				],
			},
			{
				name = "set",
				description = "Sets a member's timeout.",
				options = [
					{
						name = "member",
						description = "The member",
						option_type = InteractionOptionType::USER,
						required = true,
					},
					{
						name = "duration",
						description = "The duration to timeout, e.g. 1h",
						option_type = InteractionOptionType::STRING,
						required = true,
					},
				],
			},
		],
	)]
    async fn timeout(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    timeout
}
