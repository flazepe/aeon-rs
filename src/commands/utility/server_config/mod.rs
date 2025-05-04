mod fix_embeds;
mod logs;
mod prefix;
mod view;

use crate::structs::{command::AeonCommand, command_context::AeonCommandInput};
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

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("server-config", &[])
        .add_subcommand("fix-embeds", &[], fix_embeds::run)
        .add_subcommand("logs", &[], logs::run)
        .add_subcommand("prefix", &[], prefix::run)
        .add_subcommand("view", &[], view::run)
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Server config commands.",
        default_member_permissions = Permissions::MANAGE_GUILD,
		integration_types = [IntegrationType::GUILD_INSTALL],
        contexts = [InteractionContextType::GUILD],
        subcommands = [
			{
                name = "fix-embeds",
                description = "Fix embeds.",
                options = [
					{
						name = "enabled",
						description = "Whether to enable fix embeds",
						option_type = InteractionOptionType::BOOLEAN,
                    },
                ],
            },
            {
                name = "logs",
                description = "Logs.",
                options = [
                    {
						name = "enabled",
						description = "Whether to enable logs",
						option_type = InteractionOptionType::BOOLEAN,
                    },
					{
						name = "channel",
						description = "The channel to send logs to",
						option_type = InteractionOptionType::CHANNEL,
                        channel_types = [ChannelType::GUILD_TEXT],
                    },
                    {
						name = "ignore-bots",
						description = "Whether to ignore bots' messages",
						option_type = InteractionOptionType::BOOLEAN,
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
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(input, res)).await?;
    }

    func
}
