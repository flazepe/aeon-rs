use anyhow::{Context, Result};
use slashook::{
    commands::CommandInput,
    structs::{
        channels::{Attachment, Channel},
        guilds::Role,
        users::User,
    },
};

pub trait ArgGetters {
    fn get_string_arg(&self, arg: &str) -> Result<String>;
    fn get_i64_arg(&self, arg: &str) -> Result<i64>;
    fn get_bool_arg(&self, arg: &str) -> Result<bool>;
    fn get_user_arg(&self, arg: &str) -> Result<&User>;
    fn get_channel_arg(&self, arg: &str) -> Result<&Channel>;
    fn get_role_arg(&self, arg: &str) -> Result<&Role>;
    fn get_f64_arg(&self, arg: &str) -> Result<f64>;
    fn get_attachment_arg(&self, arg: &str) -> Result<&Attachment>;
}

impl ArgGetters for CommandInput {
    fn get_string_arg(&self, arg: &str) -> Result<String> {
        Ok(self
            .args
            .get(arg)
            .context("Could not get arg")?
            .as_string()
            .context("Could not convert arg to String")?)
    }

    fn get_i64_arg(&self, arg: &str) -> Result<i64> {
        Ok(self
            .args
            .get(arg)
            .context("Could not get arg")?
            .as_i64()
            .context("Could not convert arg to i64")?)
    }

    fn get_bool_arg(&self, arg: &str) -> Result<bool> {
        Ok(self
            .args
            .get(arg)
            .context("Could not get arg")?
            .as_bool()
            .context("Could not convert arg to bool")?)
    }

    fn get_user_arg(&self, arg: &str) -> Result<&User> {
        Ok(self
            .args
            .get(arg)
            .context("Could not get arg")?
            .as_user()
            .context("Could not convert arg to User")?)
    }

    fn get_channel_arg(&self, arg: &str) -> Result<&Channel> {
        Ok(self
            .args
            .get(arg)
            .context("Could not get arg")?
            .as_channel()
            .context("Could not convert arg to Channel")?)
    }

    fn get_role_arg(&self, arg: &str) -> Result<&Role> {
        Ok(self
            .args
            .get(arg)
            .context("Could not get arg")?
            .as_role()
            .context("Could not convert arg to Role")?)
    }

    fn get_f64_arg(&self, arg: &str) -> Result<f64> {
        Ok(self
            .args
            .get(arg)
            .context("Could not get arg")?
            .as_f64()
            .context("Could not convert arg to f64")?)
    }

    fn get_attachment_arg(&self, arg: &str) -> Result<&Attachment> {
        Ok(self
            .args
            .get(arg)
            .context("Could not get arg")?
            .as_attachment()
            .context("Could not convert arg to Attachment")?)
    }
}
