use crate::{macros::if_else, statics::MONGODB};
use anyhow::{bail, Result};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use serde::{Deserialize, Serialize};
use slashook::structs::{guilds::GuildMember, Permissions};
use std::time::{SystemTime, UNIX_EPOCH};

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

pub struct Tags {
    tags: Collection<Tag>,
}

impl Tags {
    pub fn new() -> Self {
        Self {
            tags: MONGODB.get().unwrap().collection::<Tag>("tags"),
        }
    }

    pub async fn get<T: ToString, U: ToString>(&self, name: T, guild_id: U) -> Result<Tag> {
        let name = name.to_string().to_lowercase();

        match self
            .tags
            .find_one(
                doc! {
                    "$or": [{ "name": &name }, { "aliases": name }],
                    "guild_id": guild_id.to_string(),
                },
                None,
            )
            .await?
        {
            Some(tag) => Ok(tag),
            None => bail!("Tag not found."),
        }
    }

    pub async fn search<T: ToString, U: ToString>(&self, guild_id: T, author_id: Option<U>) -> Result<Vec<Tag>> {
        let tags = self
            .tags
            .find(
                if let Some(author_id) = author_id {
                    doc! {
                        "guild_id": guild_id.to_string(),
                        "author_id": author_id.to_string(),
                    }
                } else {
                    doc! {
                        "guild_id": guild_id.to_string(),
                    }
                },
                None,
            )
            .await?
            .try_collect::<Vec<Tag>>()
            .await?;

        if_else!(tags.is_empty(), bail!("No tags found."), Ok(tags))
    }

    pub async fn create<T: ToString, U: ToString + Copy, V: ToString + Copy, W: ToString>(
        &self,
        name: T,
        guild_id: U,
        author_id: V,
        content: W,
        modifier: GuildMember,
    ) -> Result<String> {
        if !modifier
            .permissions
            .unwrap_or(Permissions::empty())
            .contains(Permissions::MANAGE_MESSAGES)
            && author_id.to_string() != "590455379931037697".to_string()
        {
            bail!("Only members with the Manage Messages permission can create tags.");
        }

        let name = Tags::validate_tag_name(name)?;

        if self.get(&name, guild_id).await.is_ok() {
            bail!("Tag already exists.");
        }

        if self
            .tags
            .count_documents(
                doc! {
                    "guild_id": guild_id.to_string(),
                },
                None,
            )
            .await?
            == 100
        {
            bail!("I'm sure 100 tags are enough for your server...");
        }

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        self.tags
            .insert_one(
                Tag {
                    _id: ObjectId::new(),
                    name,
                    aliases: vec![],
                    guild_id: guild_id.to_string(),
                    author_id: author_id.to_string(),
                    nsfw: false,
                    content: content.to_string(),
                    created_timestamp: timestamp,
                    updated_timestamp: timestamp,
                },
                None,
            )
            .await?;

        Ok("Created.".into())
    }

    pub async fn delete<T: ToString, U: ToString>(
        &self,
        name: T,
        guild_id: U,
        modifier: GuildMember,
    ) -> Result<String> {
        let tag = Tags::validate_tag_modifier(self.get(name, guild_id).await?, modifier)?;

        self.tags
            .delete_one(
                doc! {
                    "name": tag.name,
                    "guild_id": tag.guild_id,
                },
                None,
            )
            .await?;

        Ok("Gone.".into())
    }

    pub async fn edit<T: ToString, U: ToString + Copy, V: ToString, W: ToString>(
        &self,
        name: T,
        guild_id: U,
        new_name: V,
        content: W,
        modifier: GuildMember,
    ) -> Result<String> {
        let tag = Tags::validate_tag_modifier(self.get(name, guild_id).await?, modifier)?;

        let name = {
            let new_name = new_name.to_string();

            if new_name.is_empty() {
                tag.name.clone()
            } else {
                let new_name = Tags::validate_tag_name(new_name)?;

                if self.get(&new_name, guild_id).await.is_ok() {
                    bail!("Tag with that new name already exists.");
                }

                new_name
            }
        };

        let content = content.to_string();

        if name != tag.name || content != tag.content {
            self.tags
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
                    None,
                )
                .await?;

            Ok("Edited.".into())
        } else {
            bail!("No changes detected.");
        }
    }

    pub async fn toggle_alias<T: ToString, U: ToString + Copy, V: ToString>(
        &self,
        name: T,
        guild_id: U,
        alias: V,
        modifier: GuildMember,
    ) -> Result<String> {
        let mut tag = Tags::validate_tag_modifier(self.get(name, guild_id).await?, modifier)?;
        let alias = Tags::validate_tag_name(alias)?;
        let new = !tag.aliases.contains(&alias);

        if new {
            if tag.aliases.len() == 5 {
                bail!("Maximum alias amount reached.");
            }

            if self.get(&alias, guild_id).await.is_ok() {
                bail!("Tag with that new alias already exists.");
            }

            tag.aliases.push(alias.clone());
        } else {
            tag.aliases = tag
                .aliases
                .into_iter()
                .filter(|entry| entry != &alias)
                .collect::<Vec<String>>();
        }

        self.tags
            .update_one(
                doc! {
                    "name": &tag.name,
                    "guild_id": tag.guild_id,
                },
                doc! {
                    "$set": {
                        "aliases": &tag.aliases,
                        "updated_timestamp": SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
                    },
                },
                None,
            )
            .await?;

        Ok(if_else!(
            new,
            format!("Added `{}` to `{}` alias.", alias, tag.name),
            format!("Removed `{}` from `{}` alias.", alias, tag.name)
        ))
    }

    pub async fn toggle_nsfw<T: ToString, U: ToString>(
        &self,
        name: T,
        guild_id: U,
        modifier: GuildMember,
    ) -> Result<String> {
        let tag = Tags::validate_tag_modifier(self.get(name, guild_id).await?, modifier)?;
        let nsfw = !tag.nsfw;

        self.tags
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
                None,
            )
            .await?;

        Ok(format!(
            "Set tag `{}` as {}.",
            tag.name,
            if_else!(nsfw, "NSFW", "non-NSFW")
        ))
    }

    pub fn validate_tag_modifier(tag: Tag, member: GuildMember) -> Result<Tag> {
        if tag.author_id != member.user.map_or("".into(), |user| user.id)
            && !member
                .permissions
                .unwrap_or(Permissions::empty())
                .contains(Permissions::MANAGE_MESSAGES)
        {
            bail!("You're not the author of that tag. Only tag authors and members with the Manage Messages permission can update or delete tags.");
        }

        Ok(tag)
    }

    fn validate_tag_name<T: ToString>(name: T) -> Result<String> {
        let name = name.to_string().to_lowercase();

        // This should be handled by Discord but I'm adding it anyway
        if name.len() > 30 {
            bail!("Tag name length must not exceed 30 characters.");
        }

        if name.contains("`") {
            bail!("Tag name must not contain `.");
        }

        Ok(name)
    }
}
