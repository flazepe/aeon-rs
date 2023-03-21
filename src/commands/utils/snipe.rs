use crate::{constants::*, structs::gateway::cache::CACHE, traits::*, *};
use anyhow::Context;
use slashook::{
    chrono::DateTime,
    command,
    commands::*,
    structs::{channels::*, embeds::*, interactions::*, utils::File},
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
        ],
    )]
    fn snipe(input: CommandInput, res: CommandResponder) {
        match input.subcommand.as_deref().unwrap_or("") {
            "message" => {
                let channel_id = and_then_or!(
                    input.get_channel_arg("channel"),
                    |channel| Ok(channel.id.to_string()),
                    input
                        .channel_id
                        .as_ref()
                        .context("channel_id missing")?
                        .to_string()
                );

                res.send_message({
                    let cache = CACHE.lock()?;
                    let empty_vec = vec![];
                    let messages = if_else!(
                        input.get_bool_arg("edit")?,
                        &cache.edit_snipes,
                        &cache.snipes
                    )
                    .get(&channel_id)
                    .unwrap_or(&empty_vec);

                    let response = MessageResponse::from("");

                    if messages.is_empty() {
                        response.set_content(format!("{ERROR_EMOJI} no snipes found"))
                    } else if input.get_bool_arg("list")? {
                        response.add_file(File::new(
                            "snipes.txt",
                            messages
                                .into_iter()
                                .map(|message| {
                                    format!(
                                        "From {} at {}:\n\n{}",
                                        twilight_user_to_tag!(message.author),
                                        DateTime::parse_from_rfc3339(
                                            &message.timestamp.iso_8601().to_string(),
                                        )
                                        .unwrap()
                                        .to_rfc2822(),
                                        stringify_message!(&message)
                                            .split("\n")
                                            .map(|line| format!("\t{line}"))
                                            .collect::<Vec<String>>()
                                            .join("\n")
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join("\n\n"),
                        ))
                    } else {
                        let message = &messages[messages.len() - 1];

                        response.add_embed(
                            Embed::new()
                                .set_title(format!(
                                    "Latest {}snipe for <#{}>",
                                    if_else!(input.get_bool_arg("edit")?, "edit ", ""),
                                    channel_id
                                ))
                                .set_footer(
                                    twilight_user_to_tag!(message.author),
                                    input.user.avatar_url("png", 64),
                                )
                                .set_description(stringify_message!(&message))
                                .set_timestamp(DateTime::parse_from_rfc3339(
                                    &message.timestamp.iso_8601().to_string(),
                                )?),
                        )
                    }
                })
                .await?;
            }
            _ => {}
        }
    }

    snipe
}
