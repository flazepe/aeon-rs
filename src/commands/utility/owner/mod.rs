mod delete;
mod edit_server_profile;
mod eien;
mod eval;
mod request;
mod set_status;
mod status;

use crate::structs::{command::AeonCommand, command_context::AeonCommandInput};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::interactions::{ApplicationCommandOptionChoice, IntegrationType, InteractionContextType, InteractionOptionType},
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<AeonCommand> = LazyLock::new(|| {
    AeonCommand::new("owner", &["o"])
        .set_owner_only(true)
        .add_subcommand("delete", &["del"], delete::run)
        .add_subcommand("eien", &[], eien::run)
        .add_subcommand("eval", &["e", "evak"], eval::run)
        .add_subcommand("request", &["req"], request::run)
        .add_subcommand("edit-server-profile", &[], edit_server_profile::run)
        .add_subcommand("set-status", &["ss"], set_status::run)
        .add_subcommand("status", &[], status::run)
});

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Owner commands.",
        integration_types = [IntegrationType::GUILD_INSTALL, IntegrationType::USER_INSTALL],
        contexts = [InteractionContextType::GUILD, InteractionContextType::BOT_DM, InteractionContextType::PRIVATE_CHANNEL],
        subcommands = [
            {
                name = "eien",
                description = "For testing Eien.",
				options = [
					{
                        name = "command",
                        description = "The command",
                        option_type = InteractionOptionType::STRING,
						required = true,
                    },
					{
                        name = "args",
                        description = "The args",
                        option_type = InteractionOptionType::STRING,
                        required = true,
                    },
				],
            },
			{
                name = "request",
                description = "Creates a request to the Discord API.",
				options = [
					{
                        name = "endpoint",
                        description = "The endpoint",
                        option_type = InteractionOptionType::STRING,
						required = true,
                    },
					{
                        name = "body",
                        description = "The body",
                        option_type = InteractionOptionType::STRING,
                    },
					{
                        name = "method",
                        description = "The HTTP method",
                        option_type = InteractionOptionType::STRING,
						choices = [
							ApplicationCommandOptionChoice::new("GET", "GET"),
							ApplicationCommandOptionChoice::new("POST", "POST"),
							ApplicationCommandOptionChoice::new("PUT", "PUT"),
							ApplicationCommandOptionChoice::new("DELETE", "DELETE"),
							ApplicationCommandOptionChoice::new("HEAD", "HEAD"),
							ApplicationCommandOptionChoice::new("OPTIONS", "OPTIONS"),
							ApplicationCommandOptionChoice::new("CONNECT", "CONNECT"),
							ApplicationCommandOptionChoice::new("PATCH", "PATCH"),
							ApplicationCommandOptionChoice::new("TRACE", "TRACE"),
						],
                    },
				],
            },
            {
                name = "edit-server-profile",
                description = "Edits the bot's server profile.",
				options = [
                    {
                        name = "server-id",
                        description = "The server ID. Defaults to the current server",
                        option_type = InteractionOptionType::STRING,
                    },
					{
                        name = "avatar",
                        description = "The avatar to set",
                        option_type = InteractionOptionType::ATTACHMENT,
                    },
                    {
                        name = "banner",
                        description = "The banner to set",
                        option_type = InteractionOptionType::ATTACHMENT,
                    },
					{
                        name = "nickname",
                        description = "The nickname to set",
                        option_type = InteractionOptionType::STRING,
                        max_length = 32,
                    },
                    {
                        name = "about-me",
                        description = "The about me to set",
                        option_type = InteractionOptionType::STRING,
                        max_length = 190,
                    },
					{
                        name = "reset",
                        description = "The property to reset",
                        option_type = InteractionOptionType::STRING,
						choices = [
							ApplicationCommandOptionChoice::new("Avatar", "avatar"),
							ApplicationCommandOptionChoice::new("Banner", "banner"),
							ApplicationCommandOptionChoice::new("Nickname", "nickname"),
                            ApplicationCommandOptionChoice::new("About Me", "about-me"),
						],
                    },
                    {
                        name = "reset-all",
                        description = "Whether to reset all properties",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
				],
            },
            {
                name = "status",
                description = "Sends the process status.",
            },
        ],
    )]
    async fn func(input: CommandInput, res: CommandResponder) {
        COMMAND.run(AeonCommandInput::ApplicationCommand(Box::new(input), res)).await?;
    }

    func
}
