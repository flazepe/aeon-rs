mod avatar;
mod banner;

use crate::structs::{command::Command, command_context::Input};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> =
    LazyLock::new(|| Command::new("user", &[]).subcommand("avatar", &[], avatar::run).subcommand("banner", &[], banner::run));

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
		description = "Fetches various resources on a Discord user.",
		integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
		subcommands = [
			{
				name = "avatar",
				description = "Fetches a user's avatar.",
				options = [
					{
						name = "user",
						description = "The user",
						option_type = InteractionOptionType::USER,
					},
					{
						name = "force-user-avatar",
						description = "Whether to force showing user avatar instead of server avatar",
						option_type = InteractionOptionType::BOOLEAN,
					},
				],
			},
			{
				name = "banner",
				description = "Fetches a user's banner.",
				options = [
					{
						name = "user",
						description = "The user",
						option_type = InteractionOptionType::USER,
					},
				],
			},
		],
	)]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(Input::ApplicationCommand(input, res)).await?;
    }

    func
}
