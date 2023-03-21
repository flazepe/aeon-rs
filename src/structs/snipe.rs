use crate::{structs::gateway::cache::CACHE, traits::*, *};
use anyhow::{bail, Result};
use slashook::chrono::DateTime;
use slashook::commands::MessageResponse;
use slashook::structs::embeds::Embed;
use slashook::structs::utils::File;

pub struct Snipes {
    pub channel_id: String,
    pub is_edit: bool,
    pub send_list: bool,
}

impl Snipes {
    pub fn new(channel_id: &str, is_edit: &bool, send_list: &bool) -> Self {
        Self {
            channel_id: channel_id.to_string(),
            is_edit: is_edit.clone(),
            send_list: send_list.clone(),
        }
    }

    pub fn to_response(&self) -> Result<MessageResponse> {
        let cache = CACHE.lock().unwrap();
        let empty_vec = vec![];
        let snipes = if_else!(self.is_edit, &cache.edit_snipes, &cache.snipes)
            .get(&self.channel_id)
            .unwrap_or(&empty_vec);

        if snipes.is_empty() {
            bail!("no snipes found");
        }

        let mut response = MessageResponse::from(format!(
            "last {} for <#{}>:",
            plural!(
                if_else!(self.send_list, snipes.len(), 1),
                if_else!(self.is_edit, "edit snipe", "snipe")
            ),
            self.channel_id
        ));

        if self.send_list {
            response = response.add_file(File::new(
                "snipes.txt",
                snipes
                    .into_iter()
                    .map(|message| {
                        format!(
							"{} ({}) at {}:\n\n{}",
							twilight_user_to_tag!(message.author),
							message.author.id,
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
            ));
        } else {
            let snipe = &snipes[snipes.len() - 1];

            response = response.add_embed(
                Embed::new()
                    .set_footer(
                        twilight_user_to_tag!(snipe.author),
                        snipe.author.avatar_url("png", 64),
                    )
                    .set_description(stringify_message!(&snipe))
                    .set_timestamp(DateTime::parse_from_rfc3339(
                        &snipe.timestamp.iso_8601().to_string(),
                    )?),
            );
        }

        Ok(response)
    }
}

pub struct ReactionSnipes {
    pub guild_id: Option<String>,
    pub message_id: String,
}

impl ReactionSnipes {
    pub fn new(guild_id: &Option<String>, message_id: &str) -> Self {
        Self {
            guild_id: guild_id.clone(),
            message_id: message_id.to_string(),
        }
    }

    pub fn to_response(self) -> Result<MessageResponse> {
        let cache = CACHE.lock().unwrap();
        let empty_vec = vec![];
        let reaction_snipes = cache
            .reaction_snipes
            .get(&format!(
                "{}/{}",
                self.guild_id.unwrap_or("".into()),
                self.message_id
            ))
            .unwrap_or(&empty_vec);

        if reaction_snipes.is_empty() {
            bail!("no reaction snipes found");
        }

        Ok(MessageResponse::from(format!(
            "last {} for `{}`:",
            plural!(reaction_snipes.len(), "reaction snipe"),
            self.message_id
        ))
        .add_embed(Embed::new().set_description(reaction_snipes.join("\n\n"))))
    }
}
