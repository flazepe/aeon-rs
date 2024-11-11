use crate::{
    functions::label_num,
    statics::{colors::PRIMARY_COLOR, CACHE},
    structs::simple_message::SimpleMessage,
    traits::UserExt,
};
use anyhow::{bail, Result};
use slashook::{
    chrono::DateTime,
    commands::MessageResponse,
    structs::{embeds::Embed, utils::File, Permissions},
};
use std::fmt::Display;

pub struct Snipes {
    channel_id: String,
    is_edit: bool,
    send_list: bool,
    permissions: Permissions,
}

impl Snipes {
    pub fn new<T: Display>(channel_id: T, is_edit: bool, send_list: bool, permissions: Permissions) -> Self {
        Self { channel_id: channel_id.to_string(), is_edit, send_list, permissions }
    }

    pub fn to_response(&self) -> Result<MessageResponse> {
        let empty_vec = vec![];
        let snipes = if self.is_edit { &CACHE.edit_snipes } else { &CACHE.snipes }.read().unwrap();
        let snipes = snipes.get(&self.channel_id).unwrap_or(&empty_vec);

        if snipes.is_empty() {
            if !self.permissions.contains(Permissions::VIEW_CHANNEL) {
                bail!("I do not have the view channel permission to collect snipes.");
            }

            bail!("No snipes found.");
        }

        if self.send_list {
            return Ok(File::new(
                if self.is_edit { "edit-snipes.txt" } else { "snipes.txt" },
                snipes
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

        let snipe = &snipes[snipes.len() - 1];

        Ok(Embed::new()
            .set_color(PRIMARY_COLOR)?
            .set_description(SimpleMessage::from(snipe.clone()))
            .set_footer(&snipe.author.name, Some(snipe.author.display_avatar_url("png", 64)))
            .set_timestamp(DateTime::parse_from_rfc3339(&snipe.timestamp.iso_8601().to_string())?)
            .into())
    }
}

pub struct ReactionSnipes {
    guild_id: String,
    channel_id: String,
    message_id: String,
    permissions: Permissions,
}

impl ReactionSnipes {
    pub fn new<T: Display, U: Display, V: Display>(guild_id: T, channel_id: U, message_id: V, permissions: Permissions) -> Self {
        Self { guild_id: guild_id.to_string(), channel_id: channel_id.to_string(), message_id: message_id.to_string(), permissions }
    }

    pub fn to_response(&self) -> Result<MessageResponse> {
        let empty_vec = vec![];
        let reaction_snipes = CACHE.reaction_snipes.read().unwrap();
        let reaction_snipes = reaction_snipes.get(&format!("{}/{}", self.channel_id, self.message_id)).unwrap_or(&empty_vec);

        if reaction_snipes.is_empty() {
            if !self.permissions.contains(Permissions::VIEW_CHANNEL) {
                bail!("I do not have the view channel permission to collect reaction snipes.");
            }

            bail!("No reaction snipes found.");
        }

        Ok(MessageResponse::from(format!(
            "Last {} for https://discord.com/channels/{}/{}/{}",
            label_num(reaction_snipes.len(), "reaction snipe", "reaction snipes"),
            self.guild_id,
            self.channel_id,
            self.message_id,
        ))
        .add_embed(Embed::new().set_color(PRIMARY_COLOR)?.set_description(reaction_snipes.join("\n\n"))))
    }
}
