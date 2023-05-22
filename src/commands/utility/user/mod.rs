mod avatar;
mod banner;

use crate::structs::command::Command;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<Command> = Lazy::new(|| Command::new().subcommand("avatar", avatar::run).subcommand("banner", banner::run));

pub fn get_command() -> SlashookCommand {
    #[command(
		name = "user",
		description = "Fetches various resources on a Discord user.",
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
    async fn timeout(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    timeout
}
