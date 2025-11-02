use crate::{functions::now, statics::FLAZEPE_ID};
use anyhow::{Context, Result, bail};
use futures::stream::TryStreamExt;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};
use slashook::structs::{Permissions, guilds::GuildMember};
use std::fmt::Display;

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Debug)]
pub struct Tags {
    collection: Collection<Tag>,
}

impl Tags {
    pub fn new(collection: Collection<Tag>) -> Self {
        Self { collection }
    }

    pub async fn get<T: Display, U: Display>(&self, name: T, guild_id: U) -> Result<Tag> {
        let name = name.to_string().to_lowercase();

        self.collection
            .find_one(doc! {
                "$or": [{ "name": &name }, { "aliases": name }],
                "guild_id": guild_id.to_string(),
            })
            .await?
            .context("Tag not found.")
    }

    pub async fn search<T: Display, U: Display>(&self, guild_id: T, author_id: Option<U>) -> Result<Vec<Tag>> {
        let tags = self
            .collection
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
        &self,
        name: T,
        guild_id: U,
        author_id: V,
        content: W,
        modifier: &GuildMember,
    ) -> Result<String> {
        let has_permission = modifier.permissions.unwrap_or_else(Permissions::empty).contains(Permissions::MANAGE_MESSAGES);
        let is_flazepe = author_id.to_string() == FLAZEPE_ID;

        if !has_permission && !is_flazepe {
            bail!("Only members with the Manage Messages permission can create tags.");
        }

        let name = Self::validate_tag_name(name)?;

        if self.get(&name, guild_id).await.is_ok() {
            bail!("Tag already exists.");
        }

        if self.collection.count_documents(doc! { "guild_id": guild_id.to_string() }).await? == 100 {
            bail!("I'm sure 100 tags are enough for your server.");
        }

        let timestamp = now();

        self.collection
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

    pub async fn delete<T: Display, U: Display>(&self, name: T, guild_id: U, modifier: &GuildMember) -> Result<String> {
        let tag = Self::validate_tag_modifier(self.get(name, guild_id).await?, modifier)?;
        self.collection.delete_one(doc! { "name": tag.name, "guild_id": tag.guild_id }).await?;
        Ok("Gone.".into())
    }

    pub async fn edit<T: Display, U: Display + Copy, V: Display, W: Display>(
        &self,
        name: T,
        guild_id: U,
        new_name: V,
        content: W,
        modifier: &GuildMember,
    ) -> Result<String> {
        let tag = Self::validate_tag_modifier(self.get(name, guild_id).await?, modifier)?;

        let name = {
            let new_name = new_name.to_string();

            if new_name.is_empty() {
                tag.name.clone()
            } else {
                let new_name = Self::validate_tag_name(new_name)?;

                if self.get(&new_name, guild_id).await.is_ok() {
                    bail!("Tag with that new name already exists.");
                }

                new_name
            }
        };

        let content = content.to_string();

        if name == tag.name && content == tag.content {
            bail!("No changes detected.");
        }

        self.collection
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
        &self,
        name: T,
        guild_id: U,
        alias: V,
        modifier: &GuildMember,
    ) -> Result<String> {
        let mut tag = Tags::validate_tag_modifier(self.get(name, guild_id).await?, modifier)?;
        let alias = Tags::validate_tag_name(alias)?;
        let new = !tag.aliases.contains(&alias);

        if new {
            if tag.aliases.len() >= 5 {
                bail!("Maximum alias amount reached.");
            }

            if self.get(&alias, guild_id).await.is_ok() {
                bail!("Tag with that new alias already exists.");
            }

            tag.aliases.push(alias.clone());
        } else {
            tag.aliases.retain(|entry| entry != &alias)
        }

        self.collection
            .update_one(
                doc! {
                    "name": &tag.name,
                    "guild_id": tag.guild_id,
                },
                doc! {
                    "$set": {
                        "aliases": tag.aliases,
                        "updated_timestamp": now() as i64,
                    },
                },
            )
            .await?;

        if new {
            Ok(format!("Added `{alias}` to `{}` alias.", tag.name))
        } else {
            Ok(format!("Removed `{alias}` from `{}` alias.", tag.name))
        }
    }

    pub async fn toggle_nsfw<T: Display, U: Display>(&self, name: T, guild_id: U, modifier: &GuildMember) -> Result<String> {
        let tag = Self::validate_tag_modifier(self.get(name, guild_id).await?, modifier)?;
        let nsfw = !tag.nsfw;

        self.collection
            .update_one(
                doc! {
                    "name": &tag.name,
                    "guild_id": tag.guild_id,
                },
                doc! {
                    "$set": {
                        "nsfw": nsfw,
                        "updated_timestamp": now() as i64,
                    },
                },
            )
            .await?;

        Ok(format!("Set tag `{}` as {}.", tag.name, if nsfw { "NSFW" } else { "non-NSFW" }))
    }

    pub fn validate_tag_modifier(tag: Tag, member: &GuildMember) -> Result<Tag> {
        let has_permission = member.permissions.unwrap_or_else(Permissions::empty).contains(Permissions::MANAGE_MESSAGES);
        let is_author = member.user.as_ref().is_some_and(|user| tag.author_id == user.id.as_str());

        if !has_permission && !is_author {
            bail!(
                "You're not the author of that tag. Only tag authors and members with the Manage Messages permission can update or delete tags.",
            );
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
