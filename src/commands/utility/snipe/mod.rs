mod message;
mod reaction;

use crate::structs::{command::Command, command_context::Input};
use slashook::{
    command,
    commands::{Command as SlashookCommand, CommandInput, CommandResponder},
    structs::{
        channels::ChannelType,
        interactions::{IntegrationType, InteractionContextType, InteractionOptionType},
    },
};
use std::sync::LazyLock;

pub static COMMAND: LazyLock<Command> =
    LazyLock::new(|| Command::new("snipe", &[]).subcommand("message", &[], message::run).subcommand("reaction", &[], reaction::run));

pub fn get_slashook_command() -> SlashookCommand {
    #[command(
        name = COMMAND.name.clone(),
        description = "Snipes messages and reactions.",
        integration_types = [IntegrationType::GUILD_INSTALL],
        contexts = [InteractionContextType::GUILD],
        subcommands = [
            {
                name = "message",
                description = "Snipes channel's messages.",
                options = [
                    {
                        name = "channel",
                        description = "The channel",
                        option_type = InteractionOptionType::CHANNEL,
                        channel_types = [
                            ChannelType::GUILD_TEXT,
                            ChannelType::GUILD_VOICE,
                            ChannelType::GUILD_ANNOUNCEMENT,
                            ChannelType::ANNOUNCEMENT_THREAD,
                            ChannelType::GUILD_PUBLIC_THREAD,
                            ChannelType::GUILD_PRIVATE_THREAD,
                            ChannelType::GUILD_STAGE_VOICE,
                        ],
                    },
                    {
                        name = "edit",
                        description = "Whether to snipe edited messages instead",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
                    {
                        name = "list",
                        description = "Whether to send snipes as a file",
                        option_type = InteractionOptionType::BOOLEAN,
                    },
                ],
            },
            {
                name = "reaction",
                description = "Snipes a messages's reactions.",
                options = [
                    {
                        name = "message",
                        description = "The message URL or ID",
                        option_type = InteractionOptionType::STRING,
                        required = true,
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
