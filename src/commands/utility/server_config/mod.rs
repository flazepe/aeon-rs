mod fix_embeds;
mod logs;
mod prefix;
mod view;

use crate::structs::command::Command;
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        Permissions,
        channels::ChannelType,
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
    },
};
use std::sync::LazyLock;

static COMMAND: LazyLock<Command> = LazyLock::new(|| {
    Command::new()
        .subcommand("fix-embeds", fix_embeds::run)
        .subcommand("logs", logs::run)
        .subcommand("prefix", prefix::run)
        .subcommand("view", view::run)
});

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
                name = "logs",
                description = "Whether to send logs to a channel.",
                options = [
					{
						name = "channel",
						description = "The channel to send logs to. Leave unset to disable logs entirely",
						option_type = InteractionOptionType::CHANNEL,
                        channel_types = [ChannelType::GUILD_TEXT],
                    },
                ],
            },
            {
                name = "prefix",
                description = "Adds or removes a prefix.",
                options = [
					{
						name = "prefix",
						description = "The prefix to add or remove",
						option_type = InteractionOptionType::STRING,
                        max_length = 32,
                        required = true,
                        autocomplete = true,
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
