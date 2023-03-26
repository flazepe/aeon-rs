mod message;
mod reaction;

use slashook::{
    command,
    commands::{Command, CommandInput, CommandResponder},
    structs::{channels::ChannelType, interactions::InteractionOptionType},
};

pub fn get_command() -> Command {
    #[command(
        name = "snipe",
        description = "Snipes messages and reactions.",
        dm_permission = false,
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
                            ChannelType::GUILD_NEWS,
                            ChannelType::GUILD_NEWS_THREAD,
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
    async fn snipe(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref().unwrap_or("") {
            "message" => message::run(input, res).await?,
            "reaction" => reaction::run(input, res).await?,
            _ => {},
        }
    }

    snipe
}
