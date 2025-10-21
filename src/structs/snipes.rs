use crate::{
    functions::{format_timestamp, label_num, now},
    statics::{REDIS, colors::PRIMARY_EMBED_COLOR},
    structs::simple_message::SimpleMessage,
    traits::{UserAvatarExt, UserExt},
};
use anyhow::{Result, bail};
use serde_json::from_str;
use slashook::{
    chrono::DateTime,
    commands::MessageResponse,
    structs::{Permissions, embeds::Embed, utils::File},
};
use std::fmt::Display;
use twilight_model::{
    channel::{Message as TwilightMessage, message::EmojiReactionType},
    gateway::GatewayReaction,
};

pub struct Snipes {
    is_edit: bool,
    send_list: bool,
    permissions: Permissions,
    snipes: Vec<TwilightMessage>,
}

impl Snipes {
    pub async fn new<T: Display, U: Display>(guild_id: T, channel_id: U, is_edit: bool, send_list: bool, permissions: Permissions) -> Self {
        let guild_id = guild_id.to_string();
        let channel_id = channel_id.to_string();

        let key = format!("guilds_{guild_id}_channels_{channel_id}_{}", if is_edit { "edit-snipes" } else { "snipes" });
        let mut snipes = REDIS.get().unwrap().hget_many(key).await.unwrap_or_default().into_iter().collect::<Vec<(String, String)>>();
        snipes.sort_by_key(|(timestamp, _)| timestamp.parse::<u64>().unwrap_or(0));
        let snipes = snipes.iter().flat_map(|message| from_str(&message.1)).collect();

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
                    .iter()
                    .rev()
                    .map(|message| {
                        format!(
                            "{} at {}:\n\n{}",
                            message.author.label(),
                            DateTime::parse_from_rfc3339(&message.timestamp.iso_8601().to_string()).unwrap().to_rfc2822(),
                            SimpleMessage::from(message.clone())
                                .to_string()
                                .split('\n')
                                .map(|line| format!("\t{}", if line.is_empty() { "<empty>" } else { line }))
                                .collect::<Vec<String>>()
                                .join("\n"),
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n\n"),
            )
            .into());
        }

        let snipe = &self.snipes[self.snipes.len() - 1];

        Ok(Embed::new()
            .set_color(PRIMARY_EMBED_COLOR)?
            .set_description(SimpleMessage::from(snipe.clone()).to_string().chars().take(4096).collect::<String>())
            .set_footer(snipe.author.label(), Some(snipe.author.display_avatar_url("png", 64)))
            .set_timestamp(DateTime::parse_from_rfc3339(&snipe.timestamp.iso_8601().to_string())?)
            .into())
    }
}

pub struct ReactionSnipes {
    guild_id: String,
    channel_id: String,
    message_id: String,
    permissions: Permissions,
    reaction_snipes: Vec<GatewayReaction>,
}

impl ReactionSnipes {
    pub async fn new<T: Display, U: Display, V: Display>(guild_id: T, channel_id: U, message_id: V, permissions: Permissions) -> Self {
        let guild_id = guild_id.to_string();
        let channel_id = channel_id.to_string();
        let message_id = message_id.to_string();

        let mut reaction_snipes = REDIS
            .get()
            .unwrap()
            .hget_many(format!("guilds_{guild_id}_channels_{channel_id}_messages_{message_id}_reaction-snipes"))
            .await
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<(String, String)>>();
        reaction_snipes.sort_by_key(|(timestamp, _)| timestamp.parse::<u64>().unwrap_or(0));
        let reaction_snipes = reaction_snipes.iter().flat_map(|reaction| from_str(&reaction.1)).collect();

        Self { guild_id, channel_id, message_id, permissions, reaction_snipes }
    }

    pub fn to_response(&self) -> Result<MessageResponse> {
        if self.reaction_snipes.is_empty() {
            if !self.permissions.contains(Permissions::VIEW_CHANNEL) {
                bail!("I do not have the view channel permission to collect reaction snipes.");
            }

            bail!("No reaction snipes found.");
        }

        Ok(MessageResponse::from(format!(
            "Last {} for https://discord.com/channels/{}/{}/{}",
            label_num(self.reaction_snipes.len(), "reaction snipe", "reaction snipes"),
            self.guild_id,
            self.channel_id,
            self.message_id,
        ))
        .add_embed(
            Embed::new().set_color(PRIMARY_EMBED_COLOR)?.set_description(
                self.reaction_snipes
                    .iter()
                    .map(|reaction| {
                        format!(
                            "<@{}> - {}\n{}",
                            reaction.user_id,
                            match &reaction.emoji {
                                EmojiReactionType::Custom { name, id, .. } =>
                                    format!("[{}](https://cdn.discordapp.com/emojis/{id})", name.as_deref().unwrap_or("<unknown>")),
                                EmojiReactionType::Unicode { name } => name.clone(),
                            },
                            format_timestamp(now(), true),
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n\n"),
            ),
        ))
    }
}
