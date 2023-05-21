mod avatar;
mod banner;

use crate::structs::command::AeonCommand;
use once_cell::sync::Lazy;
use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::interactions::InteractionOptionType,
};

static COMMAND: Lazy<AeonCommand> = Lazy::new(|| AeonCommand::new().subcommand("avatar", avatar::run).subcommand("banner", banner::run));

pub fn get_command() -> Command {
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
