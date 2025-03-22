mod fix_embeds;
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

static COMMAND: LazyLock<Command> =
    LazyLock::new(|| Command::new().subcommand("fix-embeds", fix_embeds::run).subcommand("view", view::run));

pub fn get_command() -> SlashookCommand {
    #[command(
        name = "server-config",
        description = "Server config commands.",
        default_member_permissions = Permissions::MANAGE_GUILD,
		integration_types = [IntegrationType::GUILD_INSTALL],
        contexts = [InteractionContextType::GUILD],
        subcommands = [
			{
                name = "fix-embeds",
                description = "Whether to fix embeds.",
                options = [
					{
						name = "enabled",
						description = "Whether to fix embeds",
						option_type = InteractionOptionType::BOOLEAN,
                        required = true,
                    },
                ],
            },
            {
                name = "view",
                description = "Sends the server config.",
            },
        ],
    )]
    async fn server_config(input: CommandInput, res: CommandResponder) {
        COMMAND.run(input, res).await?;
    }

    server_config
}
