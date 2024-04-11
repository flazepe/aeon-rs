use crate::{
    functions::label_num,
    statics::{colors::PRIMARY_COLOR, CACHE},
    structs::stringified_message::StringifiedMessage,
    traits::AvatarUrl,
};
use anyhow::{bail, Result};
use slashook::{
    chrono::DateTime,
    commands::MessageResponse,
    structs::{embeds::Embed, utils::File},
};

pub struct Snipes {
    channel_id: String,
    is_edit: bool,
    send_list: bool,
}

impl Snipes {
    pub fn new<T: ToString>(channel_id: T, is_edit: bool, send_list: bool) -> Self {
        Self { channel_id: channel_id.to_string(), is_edit, send_list }
    }

    pub fn to_response(&self) -> Result<MessageResponse> {
        let empty_vec = vec![];

        let snipes = match self.is_edit {
            true => &CACHE.edit_snipes,
            false => &CACHE.snipes,
        }
        .read()
        .unwrap();

        let snipes = snipes.get(&self.channel_id).unwrap_or(&empty_vec);

        if snipes.is_empty() {
            bail!("No snipes found.");
        }

        Ok(match self.send_list {
            true => File::new(
                match self.is_edit {
                    true => "edit-snipes.txt",
                    false => "snipes.txt",
                },
                snipes
                    .iter()
                    .rev()
                    .map(|message| {
                        format!(
                            "{} ({}) at {}:\n\n{}",
                            message.author.name,
                            message.author.id,
                            DateTime::parse_from_rfc3339(&message.timestamp.iso_8601().to_string()).unwrap().to_rfc2822(),
                            StringifiedMessage::from(message.clone())
                                .to_string()
                                .split('\n')
                                .map(|line| format!(
                                    "\t{}",
                                    match line.is_empty() {
                                        true => "<empty>",
                                        false => line,
                                    },
                                ))
                                .collect::<Vec<_>>()
                                .join("\n"),
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n\n"),
            )
            .into(),
            false => {
                let snipe = &snipes[snipes.len() - 1];

                Embed::new()
                    .set_color(PRIMARY_COLOR)?
                    .set_description(StringifiedMessage::from(snipe.clone()))
                    .set_footer(&snipe.author.name, Some(snipe.author.display_avatar_url("png", 64)))
                    .set_timestamp(DateTime::parse_from_rfc3339(&snipe.timestamp.iso_8601().to_string())?)
                    .into()
            },
        })
    }
}

pub struct ReactionSnipes {
    pub guild_id: String,
    pub channel_id: String,
    pub message_id: String,
}

impl ReactionSnipes {
    pub fn new<T: ToString, U: ToString, V: ToString>(guild_id: T, channel_id: U, message_id: V) -> Self {
        Self { guild_id: guild_id.to_string(), channel_id: channel_id.to_string(), message_id: message_id.to_string() }
    }

    pub fn to_response(&self) -> Result<MessageResponse> {
        let empty_vec = vec![];
        let reaction_snipes = CACHE.reaction_snipes.read().unwrap();
        let reaction_snipes = reaction_snipes.get(&format!("{}/{}", self.channel_id, self.message_id)).unwrap_or(&empty_vec);

        if reaction_snipes.is_empty() {
            bail!("No reaction snipes found.");
        }

        Ok(MessageResponse::from(format!(
            "Last {} for https://discord.com/channels/{}/{}/{}",
            label_num(reaction_snipes.len(), "reaction snipe"),
            self.guild_id,
            self.channel_id,
            self.message_id,
        ))
        .add_embed(Embed::new().set_color(PRIMARY_COLOR)?.set_description(reaction_snipes.join("\n\n"))))
    }
}
