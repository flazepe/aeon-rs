use crate::{
    macros::{if_else, plural, stringify_message, twilight_user_to_tag},
    statics::{colors::PRIMARY_COLOR, CACHE},
    traits::AvatarURL,
};
use anyhow::{bail, Result};
use slashook::{
    chrono::DateTime,
    commands::MessageResponse,
    structs::{embeds::Embed, utils::File},
};

pub struct Snipes {
    pub channel_id: String,
    pub is_edit: bool,
    pub send_list: bool,
}

impl Snipes {
    pub fn new<T: ToString>(channel_id: T, is_edit: bool, send_list: bool) -> Self {
        Self {
            channel_id: channel_id.to_string(),
            is_edit,
            send_list,
        }
    }

    pub fn to_response(&self) -> Result<MessageResponse> {
        let empty_vec = vec![];
        let snipes = if_else!(self.is_edit, &CACHE.edit_snipes, &CACHE.snipes)
            .lock()
            .unwrap();
        let snipes = snipes.get(&self.channel_id).unwrap_or(&empty_vec);

        if snipes.is_empty() {
            bail!("No snipes found.");
        }

        let response = MessageResponse::from("");

        if self.send_list {
            return Ok(response.add_file(File::new(
                format!("{}snipes.txt", if_else!(self.is_edit, "edit-", "")),
                snipes
                    .into_iter()
                    .map(|message| {
                        format!(
                            "{} ({}) at {}:\n\n{}",
                            twilight_user_to_tag!(message.author),
                            message.author.id,
                            DateTime::parse_from_rfc3339(&message.timestamp.iso_8601().to_string(),)
                                .unwrap()
                                .to_rfc2822(),
                            stringify_message!(&message)
                                .split("\n")
                                .map(|line| format!("\t{}", if_else!(line.is_empty(), "<empty>", line)))
                                .collect::<Vec<String>>()
                                .join("\n")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n\n"),
            )));
        }

        let snipe = &snipes[snipes.len() - 1];

        return Ok(response.add_embed(
            Embed::new()
                .set_color(PRIMARY_COLOR)?
                .set_description(stringify_message!(&snipe))
                .set_footer(twilight_user_to_tag!(snipe.author), snipe.author.avatar_url("png", 64))
                .set_timestamp(DateTime::parse_from_rfc3339(&snipe.timestamp.iso_8601().to_string())?),
        ));
    }
}

pub struct ReactionSnipes {
    pub guild_id: String,
    pub message_id: String,
}

impl ReactionSnipes {
    pub fn new<T: ToString, U: ToString>(guild_id: T, message_id: U) -> Self {
        Self {
            guild_id: guild_id.to_string(),
            message_id: message_id.to_string(),
        }
    }

    pub fn to_response(self) -> Result<MessageResponse> {
        let empty_vec = vec![];
        let reaction_snipes = CACHE.reaction_snipes.lock().unwrap();
        let reaction_snipes = reaction_snipes
            .get(&format!("{}/{}", self.guild_id, self.message_id))
            .unwrap_or(&empty_vec);

        if reaction_snipes.is_empty() {
            bail!("No reaction snipes found.");
        }

        Ok(MessageResponse::from(format!(
            "Last {} for `{}`:",
            plural!(reaction_snipes.len(), "reaction snipe"),
            self.message_id
        ))
        .add_embed(
            Embed::new()
                .set_color(PRIMARY_COLOR)?
                .set_description(reaction_snipes.join("\n\n")),
        ))
    }
}