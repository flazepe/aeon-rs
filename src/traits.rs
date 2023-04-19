use anyhow::{Context, Result};
use slashook::{
    commands::CommandInput,
    structs::{
        channels::{Attachment, Channel},
        guilds::Role,
        users::User,
    },
};
use std::fmt::Display;
use twilight_model::user::User as TwilightUser;

pub trait ArgGetters {
    fn get_string_arg<T: ToString>(&self, arg: T) -> Result<String>;
    fn get_i64_arg<T: ToString>(&self, arg: T) -> Result<i64>;
    fn get_bool_arg<T: ToString>(&self, arg: T) -> Result<bool>;
    fn get_user_arg<T: ToString>(&self, arg: T) -> Result<&User>;
    fn get_channel_arg<T: ToString>(&self, arg: T) -> Result<&Channel>;
    fn get_role_arg<T: ToString>(&self, arg: T) -> Result<&Role>;
    fn get_f64_arg<T: ToString>(&self, arg: T) -> Result<f64>;
    fn get_attachment_arg<T: ToString>(&self, arg: T) -> Result<&Attachment>;
}

impl ArgGetters for CommandInput {
    fn get_string_arg<T: ToString>(&self, arg: T) -> Result<String> {
        Ok(self
            .args
            .get(&arg.to_string())
            .context("Could not get arg.")?
            .as_string()
            .context("Could not convert arg to String.")?)
    }

    fn get_i64_arg<T: ToString>(&self, arg: T) -> Result<i64> {
        Ok(self
            .args
            .get(&arg.to_string())
            .context("Could not get arg.")?
            .as_i64()
            .context("Could not convert arg to i64.")?)
    }

    fn get_bool_arg<T: ToString>(&self, arg: T) -> Result<bool> {
        Ok(self
            .args
            .get(&arg.to_string())
            .context("Could not get arg.")?
            .as_bool()
            .context("Could not convert arg to bool.")?)
    }

    fn get_user_arg<T: ToString>(&self, arg: T) -> Result<&User> {
        Ok(self
            .args
            .get(&arg.to_string())
            .context("Could not get arg.")?
            .as_user()
            .context("Could not convert arg to User.")?)
    }

    fn get_channel_arg<T: ToString>(&self, arg: T) -> Result<&Channel> {
        Ok(self
            .args
            .get(&arg.to_string())
            .context("Could not get arg.")?
            .as_channel()
            .context("Could not convert arg to Channel.")?)
    }

    fn get_role_arg<T: ToString>(&self, arg: T) -> Result<&Role> {
        Ok(self
            .args
            .get(&arg.to_string())
            .context("Could not get arg.")?
            .as_role()
            .context("Could not convert arg to Role.")?)
    }

    fn get_f64_arg<T: ToString>(&self, arg: T) -> Result<f64> {
        Ok(self
            .args
            .get(&arg.to_string())
            .context("Could not get arg.")?
            .as_f64()
            .context("Could not convert arg to f64.")?)
    }

    fn get_attachment_arg<T: ToString>(&self, arg: T) -> Result<&Attachment> {
        Ok(self
            .args
            .get(&arg.to_string())
            .context("Could not get arg.")?
            .as_attachment()
            .context("Could not convert arg to Attachment.")?)
    }
}

pub trait AvatarURL {
    fn avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> Option<String>;
    fn display_avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> String;
}

impl AvatarURL for User {
    fn avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> Option<String> {
        self.avatar_url(format, size)
    }

    fn display_avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> String {
        match self.avatar_url(format, size) {
            Some(avatar_url) => avatar_url,
            None => format!(
                "https://cdn.discordapp.com/embed/avatars/{}.png",
                self.discriminator.parse::<u64>().unwrap() % 5,
            ),
        }
    }
}

impl AvatarURL for TwilightUser {
    fn avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> Option<String> {
        self.avatar.as_ref().map(|a| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{a}.{format}?size={size}",
                self.id,
            )
        })
    }

    fn display_avatar_url<T: Display, U: Display>(&self, format: T, size: U) -> String {
        match self.avatar_url(format, size) {
            Some(avatar_url) => avatar_url,
            None => format!(
                "https://cdn.discordapp.com/embed/avatars/{}.png",
                self.discriminator % 5,
            ),
        }
    }
}

pub trait LimitedVec<T> {
    fn push_limited(&mut self, value: T, limit: usize);
}

impl<T> LimitedVec<T> for Vec<T> {
    fn push_limited(&mut self, value: T, limit: usize) {
        self.push(value);

        let length = self.len();

        if length > limit {
            self.drain(..length - limit);
        }
    }
}

pub trait Tag {
    fn tag(&self) -> String;
}

impl Tag for User {
    fn tag(&self) -> String {
        format!("{}#{}", self.username, self.discriminator)
    }
}

impl Tag for TwilightUser {
    fn tag(&self) -> String {
        format!("{}#{}", self.name, self.discriminator())
    }
}
