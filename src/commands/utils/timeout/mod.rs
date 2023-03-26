pub mod remove;
pub mod set;

use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::{interactions::InteractionOptionType, Permissions},
};

pub fn get_command() -> Command {
    #[command(
		name = "timeout",
		description = "Manages members' timeout.",
		default_member_permissions = Permissions::MODERATE_MEMBERS,
		dm_permission = false,
		subcommands = [
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
		],
	)]
    async fn timeout(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref().unwrap_or("") {
            "set" => set::run(input, res).await?,
            "remove" => remove::run(input, res).await?,
            _ => {},
        }
    }

    timeout
}
