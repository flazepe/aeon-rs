mod remove;
mod set;

use crate::structs::{command::Command, command_context::Input};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        Permissions,
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
    },
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> =
    LazyLock::new(|| Command::new("timeout", &["mute"]).subcommand("remove", &[], remove::run).subcommand("set", &[], set::run));

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
		description = "Manages members' timeout.",
		default_member_permissions = Permissions::MODERATE_MEMBERS,
		integration_types = [IntegrationType::GUILD_INSTALL],
        contexts = [InteractionContextType::GUILD],
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
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(Input::ApplicationCommand { input, res }).await?;
    }

    func
}
