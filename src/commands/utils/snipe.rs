use crate::{statics::emojis::*, structs::snipe::*, traits::*, *};
use slashook::{
    command,
    commands::*,
    structs::{channels::*, interactions::*},
};

pub fn get_command() -> Command {
    #[command(
        name = "snipe",
        description = "Snipes messages and reactions.",
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
    fn snipe(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref().unwrap_or("") {
            "message" => {
                match Snipes::new(
                    and_then_or!(
                        input.get_channel_arg("channel"),
                        |channel| Ok(&channel.id),
                        input.channel_id.as_ref().unwrap()
                    ),
                    input.get_bool_arg("edit")?,
                    input.get_bool_arg("list")?,
                )
                .to_response()
                {
                    Ok(response) => {
                        res.send_message(response).await?;
                    }
                    Err(error) => {
                        res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                    }
                }
            }
            "reaction" => {
                let message = input.get_string_arg("message")?;

                match ReactionSnipes::new(input.guild_id, message.split("/").last().unwrap_or(""))
                    .to_response()
                {
                    Ok(response) => {
                        res.send_message(response).await?;
                    }
                    Err(error) => {
                        res.send_message(format!("{ERROR_EMOJI} {error}")).await?;
                    }
                }
            }
            _ => {}
        }
    }

    snipe
}
