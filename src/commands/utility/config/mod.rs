mod update;
mod view;

use crate::structs::command::Command;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
        Permissions,
    },
};
use std::sync::LazyLock;

static COMMAND: LazyLock<Command> = LazyLock::new(|| Command::new().subcommand("update", update::run).subcommand("view", view::run));

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "config",
        description = "Server config commands.",
        default_member_permissions = Permissions::MANAGE_GUILD,
		integration_types = [IntegrationType::GUILD_INSTALL],
        contexts = [InteractionContextType::GUILD],
        subcommands = [
			{
                name = "view",
                description = "Sends the server config.",
            },
			{
                name = "update",
                description = "Updates the server config.",
                options = [
					{
						name = "fix-embeds",
						description = "Whether to fix embeds",
						option_type = InteractionOptionType::BOOLEAN,
					},
                ],
            },
        ],
    )]
    async fn config(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    config
}
