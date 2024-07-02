use crate::statics::{COLLECTIONS, FLAZEPE_ID};
use anyhow::{bail, Result};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use slashook::structs::{guilds::GuildMember, Permissions};
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Deserialize, Serialize)]
pub struct Tag {
    pub _id: ObjectId,
    pub name: String,
    pub guild_id: String,
    pub author_id: String,
    pub aliases: Vec<String>,
    pub nsfw: bool,
    pub content: String,
    pub created_timestamp: u64,
    pub updated_timestamp: u64,
}

pub struct Tags;

impl Tags {
    pub async fn get<T: Display, U: Display>(name: T, guild_id: U) -> Result<Tag> {
        let name = name.to_string().to_lowercase();

        match COLLECTIONS
            .tags
            .find_one(doc! {
                "$or": [{ "name": &name }, { "aliases": name }],
                "guild_id": guild_id.to_string(),
            })
            .await?
        {
            Some(tag) => Ok(tag),
            None => bail!("Tag not found."),
        }
    }

    pub async fn search<T: Display, U: Display>(guild_id: T, author_id: Option<U>) -> Result<Vec<Tag>> {
        let tags = COLLECTIONS
            .tags
            .find(match author_id {
                Some(author_id) => doc! {
                    "guild_id": guild_id.to_string(),
                    "author_id": author_id.to_string(),
                },
                None => doc! {
                    "guild_id": guild_id.to_string(),
                },
            })
            .await?
            .try_collect::<Vec<Tag>>()
            .await?;

        if tags.is_empty() {
            bail!("No tags found.");
        }

        Ok(tags)
    }

    pub async fn create<T: Display, U: Display + Copy, V: Display + Copy, W: Display>(
        name: T,
        guild_id: U,
        author_id: V,
        content: W,
        modifier: &GuildMember,
    ) -> Result<String> {
        if !modifier.permissions.unwrap_or_else(Permissions::empty).contains(Permissions::MANAGE_MESSAGES)
            && author_id.to_string() != FLAZEPE_ID
        {
            bail!("Only members with the Manage Messages permission can create tags.");
        }

        let name = Self::validate_tag_name(name)?;

        if Self::get(&name, guild_id).await.is_ok() {
            bail!("Tag already exists.");
        }

        if COLLECTIONS.tags.count_documents(doc! { "guild_id": guild_id.to_string() }).await? == 100 {
            bail!("I'm sure 100 tags are enough for your server.");
        }

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        COLLECTIONS
            .tags
            .insert_one(Tag {
                _id: ObjectId::new(),
                name,
                aliases: vec![],
                guild_id: guild_id.to_string(),
                author_id: author_id.to_string(),
                nsfw: false,
                content: content.to_string(),
                created_timestamp: timestamp,
                updated_timestamp: timestamp,
            })
            .await?;

        Ok("Created.".into())
    }

    pub async fn delete<T: Display, U: Display>(name: T, guild_id: U, modifier: &GuildMember) -> Result<String> {
        let tag = Self::validate_tag_modifier(Self::get(name, guild_id).await?, modifier)?;
        COLLECTIONS.tags.delete_one(doc! { "name": tag.name, "guild_id": tag.guild_id }).await?;
        Ok("Gone.".into())
    }

    pub async fn edit<T: Display, U: Display + Copy, V: Display, W: Display>(
        name: T,
        guild_id: U,
        new_name: V,
        content: W,
        modifier: &GuildMember,
    ) -> Result<String> {
        let tag = Self::validate_tag_modifier(Self::get(name, guild_id).await?, modifier)?;

        let name = {
            let new_name = new_name.to_string();

            match new_name.is_empty() {
                true => tag.name.clone(),
                false => {
                    let new_name = Self::validate_tag_name(new_name)?;

                    if Self::get(&new_name, guild_id).await.is_ok() {
                        bail!("Tag with that new name already exists.");
                    }

                    new_name
                },
            }
        };

        let content = content.to_string();

        if name == tag.name && content == tag.content {
            bail!("No changes detected.");
        }

        COLLECTIONS
            .tags
            .update_one(
                doc! {
                    "name": tag.name,
                    "guild_id": tag.guild_id,
                },
                doc! {
                    "$set": {
                        "name": name,
                        "content": content,
                    },
                },
            )
            .await?;

        Ok("Edited.".into())
    }

    pub async fn toggle_alias<T: Display, U: Display + Copy, V: Display>(
        name: T,
        guild_id: U,
        alias: V,
        modifier: &GuildMember,
    ) -> Result<String> {
        let mut tag = Tags::validate_tag_modifier(Self::get(name, guild_id).await?, modifier)?;
        let alias = Tags::validate_tag_name(alias)?;
        let new = !tag.aliases.contains(&alias);

        match new {
            true => {
                if tag.aliases.len() >= 5 {
                    bail!("Maximum alias amount reached.");
                }

                if Self::get(&alias, guild_id).await.is_ok() {
                    bail!("Tag with that new alias already exists.");
                }

                tag.aliases.push(alias.clone());
            },
            false => tag.aliases.retain(|entry| entry != &alias),
        }

        COLLECTIONS
            .tags
            .update_one(
                doc! {
                    "name": &tag.name,
                    "guild_id": tag.guild_id,
                },
                doc! {
                    "$set": {
                        "aliases": tag.aliases,
                        "updated_timestamp": SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
                    },
                },
            )
            .await?;

        Ok(match new {
            true => format!("Added `{}` to `{}` alias.", alias, tag.name),
            false => format!("Removed `{}` from `{}` alias.", alias, tag.name),
        })
    }

    pub async fn toggle_nsfw<T: Display, U: Display>(name: T, guild_id: U, modifier: &GuildMember) -> Result<String> {
        let tag = Self::validate_tag_modifier(Self::get(name, guild_id).await?, modifier)?;
        let nsfw = !tag.nsfw;

        COLLECTIONS
            .tags
            .update_one(
                doc! {
                    "name": &tag.name,
                    "guild_id": tag.guild_id,
                },
                doc! {
                    "$set": {
                        "nsfw": nsfw,
                        "updated_timestamp": SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
                    },
                },
            )
            .await?;

        Ok(format!(
            "Set tag `{}` as {}.",
            tag.name,
            match nsfw {
                true => "NSFW",
                false => "non-NSFW",
            },
        ))
    }

    pub fn validate_tag_modifier(tag: Tag, member: &GuildMember) -> Result<Tag> {
        if tag.author_id != member.user.as_ref().map_or_else(|| "".into(), |user| user.id.clone())
            && !member.permissions.unwrap_or_else(Permissions::empty).contains(Permissions::MANAGE_MESSAGES)
        {
            bail!("You're not the author of that tag. Only tag authors and members with the Manage Messages permission can update or delete tags.");
        }

        Ok(tag)
    }

    fn validate_tag_name<T: Display>(name: T) -> Result<String> {
        let name = name.to_string().to_lowercase();

        // This should be handled by Discord but I'm adding it anyway
        if name.len() > 32 {
            bail!("Tag name length must not exceed 32 characters.");
        }

        if name.contains('`') {
            bail!("Tag name must not contain `.");
        }

        Ok(name)
    }
}
