use crate::{
    statics::{REDIS, REST, colors::PRIMARY_EMBED_COLOR},
    structs::{database::redis::keys::RedisKey, simple_message::SimpleMessage},
    traits::{EmojiReactionExt, UserAvatarExt, UserExt},
};
use anyhow::{Context, Result, bail};
use slashook::{
    chrono::DateTime,
    commands::MessageResponse,
    structs::{Permissions, embeds::Embed, users::User, utils::File},
};
use std::{collections::BTreeMap, fmt::Display};
use twilight_model::{channel::Message as TwilightMessage, gateway::GatewayReaction};

pub struct Snipes {
    is_edit: bool,
    send_list: bool,
    permissions: Permissions,
    snipes: BTreeMap<u64, TwilightMessage>,
}

impl Snipes {
    pub async fn new<T: Display, U: Display>(guild_id: T, channel_id: U, is_edit: bool, send_list: bool, permissions: Permissions) -> Self {
        let guild_id = guild_id.to_string();
        let channel_id = channel_id.to_string();

        let redis = REDIS.get().unwrap();
        let key = if is_edit {
            RedisKey::GuildChannelEditSnipes(guild_id, channel_id)
        } else {
            RedisKey::GuildChannelSnipes(guild_id, channel_id)
        };
        let snipes = redis.hget_many::<u64, TwilightMessage>(&key).await.unwrap_or_default();

        Self { is_edit, send_list, permissions, snipes }
    }

    pub fn to_response(&self) -> Result<MessageResponse> {
        if self.snipes.is_empty() {
            if !self.permissions.contains(Permissions::VIEW_CHANNEL) {
                bail!("I do not have the view channel permission to collect snipes.");
            }

            bail!("No snipes found.");
        }

        if self.send_list {
            return Ok(File::new(
                if self.is_edit { "edit-snipes.txt" } else { "snipes.txt" },
                self.snipes
                    .values()
                    .rev()
                    .map(|message| {
                        let author_label = message.author.label();
                        let date = DateTime::parse_from_rfc3339(&message.timestamp.iso_8601().to_string()).unwrap().to_rfc2822();
                        let stringified_message = SimpleMessage::from(message.clone())
                            .to_string()
                            .split('\n')
                            .map(|line| format!("\t{}", if line.is_empty() { "<empty>" } else { line }))
                            .collect::<Vec<String>>()
                            .join("\n");

                        format!("{author_label} at {date}:\n\n{stringified_message}")
                    })
                    .collect::<Vec<String>>()
                    .join("\n\n"),
            )
            .into());
        }

        let snipe = self.snipes.values().last().context("Could not get last snipe.")?;

        let embed = Embed::new()
            .set_color(PRIMARY_EMBED_COLOR)?
            .set_description(SimpleMessage::from(snipe.clone()).to_string().chars().take(4096).collect::<String>())
            .set_footer(snipe.author.label(), Some(snipe.author.display_avatar_url("png", 64)))
            .set_timestamp(DateTime::parse_from_rfc3339(&snipe.timestamp.iso_8601().to_string())?);

        Ok(embed.into())
    }
}

pub struct ReactionSnipes {
    send_list: bool,
    permissions: Permissions,
    reaction_snipes: BTreeMap<u64, GatewayReaction>,
}

impl ReactionSnipes {
    pub async fn new<T: Display, U: Display>(guild_id: T, channel_id: U, send_list: bool, permissions: Permissions) -> Self {
        let guild_id = guild_id.to_string();
        let channel_id = channel_id.to_string();

        let redis = REDIS.get().unwrap();
        let key = RedisKey::GuildChannelReactionSnipes(guild_id.clone(), channel_id.clone());
        let reaction_snipes = redis.hget_many::<u64, GatewayReaction>(&key).await.unwrap_or_default().into_iter().collect();

        Self { permissions, send_list, reaction_snipes }
    }

    pub async fn to_response(&self) -> Result<MessageResponse> {
        if self.reaction_snipes.is_empty() {
            if !self.permissions.contains(Permissions::VIEW_CHANNEL) {
                bail!("I do not have the view channel permission to collect reaction snipes.");
            }

            bail!("No reaction snipes found.");
        }

        if self.send_list {
            return Ok(File::new(
                "reaction-snipes.txt",
                self.reaction_snipes
                    .iter()
                    .rev()
                    .map(|(timestamp, reaction)| {
                        let user_id = reaction.user_id;
                        let emoji = reaction.emoji.label();

                        let guild_id = reaction.guild_id.unwrap();
                        let channel_id = reaction.channel_id;
                        let message_id = reaction.message_id;
                        let message_url = format!("https://discord.com/channels/{guild_id}/{channel_id}/{message_id}");

                        let date = DateTime::from_timestamp(*timestamp as i64, 0).unwrap().to_rfc2822();

                        format!("- <@{user_id}> unreacted emoji {emoji} from message {message_url} at {date}")
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
            .into());
        }

        let (timestamp, reaction_snipe) = self.reaction_snipes.iter().last().context("Could not get last reaction snipe.")?;
        let Some(guild_id) = reaction_snipe.guild_id else { bail!("Could not get guild ID from reaction.") };
        let channel_id = reaction_snipe.channel_id;
        let message_id = reaction_snipe.message_id;
        let user_id = reaction_snipe.user_id;

        let mut embed = Embed::new()
            .set_color(PRIMARY_EMBED_COLOR)?
            .set_thumbnail(reaction_snipe.emoji.get_image_url())
            .set_title("Message")
            .set_url(format!("https://discord.com/channels/{guild_id}/{channel_id}/{message_id}"))
            .set_description(reaction_snipe.emoji.label())
            .set_timestamp(DateTime::from_timestamp(*timestamp as i64, 0).context("Could not get reaction timestamp as DateTime.")?);

        if let Ok(user) = User::fetch(&REST, user_id).await {
            embed = embed.set_footer(user.label(), Some(user.display_avatar_url("png", Some("gif"), 64)));
        } else {
            embed = embed.set_footer(format!("User ID {user_id}"), None::<String>);
        }

        Ok(embed.into())
    }
}
