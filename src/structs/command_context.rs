use crate::statics::{
    emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
    CACHE,
};
use anyhow::{bail, Context, Result};
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::{
        channels::{Attachment, Channel},
        guilds::Role,
        interactions::ApplicationCommandOptionChoice,
        users::User,
    },
};
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct CommandContext {
    pub input: CommandInput,
    pub res: CommandResponder,
    verified: bool,
}

impl CommandContext {
    pub fn new(input: CommandInput, res: CommandResponder) -> Self {
        Self { input, res, verified: false }
    }

    pub async fn verify(mut self) -> Result<Self> {
        // Ignore verification for autocomplete
        if self.input.is_autocomplete() {
            return Ok(self);
        }

        self.verified = true;

        if CACHE.cooldowns.read().unwrap().get(&self.input.user.id).unwrap_or(&0) > &SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
        {
            self.respond_error("You are under a cooldown. Try again later.", true).await?;
            bail!("User is under a cooldown.");
        }

        // Only add cooldown to non-search commands
        if !self.get_bool_arg("search").unwrap_or(false) {
            CACHE
                .cooldowns
                .write()
                .unwrap()
                .insert(self.input.user.id.clone(), SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3);
        };

        if let Some(interaction) = self.input.message.as_ref().and_then(|message| message.interaction.as_ref()) {
            if self.input.user.id != interaction.user.id {
                self.respond_error("This isn't your interaction.", true).await?;
                bail!("User is not the interaction initiator.");
            }
        }

        Ok(self)
    }

    pub async fn respond<T: Into<MessageResponse>>(&self, response: T, ephemeral: bool) -> Result<()> {
        if !self.verified {
            bail!("Interaction isn't verified.");
        }

        let response = response.into().set_ephemeral(ephemeral);

        match self.input.message.is_some() && !ephemeral {
            true => self.res.update_message(response).await?,
            false => self.res.send_message(response).await?,
        };

        Ok(())
    }

    pub async fn respond_error<T: Display>(&self, response: T, ephemeral: bool) -> Result<()> {
        self.respond(MessageResponse::from(format!("{ERROR_EMOJI} {response}")).clear_embeds().clear_attachments(), ephemeral).await
    }

    pub async fn respond_success<T: Display>(&self, response: T, ephemeral: bool) -> Result<()> {
        self.respond(MessageResponse::from(format!("{SUCCESS_EMOJI} {response}")).clear_embeds().clear_attachments(), ephemeral).await
    }

    pub async fn autocomplete<T: Iterator<Item = (K, V)>, K: ToString, V: ToString>(&self, iter: T) -> Result<()> {
        let value = self
            .input
            .args
            .get(self.input.focused.as_ref().context("Missing focused arg.")?)
            .context("Could not get focused arg.")?
            .as_string()
            .context("Could not convert focused arg to String.")?
            .to_lowercase();

        Ok(self
            .res
            .autocomplete(
                iter.filter(|(k, v)| k.to_string().to_lowercase().contains(&value) || v.to_string().to_lowercase().contains(&value))
                    .map(|(k, v)| ApplicationCommandOptionChoice::new(v.to_string(), k.to_string()))
                    .take(25)
                    .collect(),
            )
            .await?)
    }

    pub fn get_string_arg<T: ToString>(&self, arg: T) -> Result<String> {
        self.input.args.get(&arg.to_string()).context("Could not get arg.")?.as_string().context("Could not convert arg to String.")
    }

    pub fn get_i64_arg<T: ToString>(&self, arg: T) -> Result<i64> {
        self.input.args.get(&arg.to_string()).context("Could not get arg.")?.as_i64().context("Could not convert arg to i64.")
    }

    pub fn get_bool_arg<T: ToString>(&self, arg: T) -> Result<bool> {
        self.input.args.get(&arg.to_string()).context("Could not get arg.")?.as_bool().context("Could not convert arg to bool.")
    }

    pub fn get_user_arg<T: ToString>(&self, arg: T) -> Result<&User> {
        self.input.args.get(&arg.to_string()).context("Could not get arg.")?.as_user().context("Could not convert arg to User.")
    }

    pub fn get_channel_arg<T: ToString>(&self, arg: T) -> Result<&Channel> {
        self.input.args.get(&arg.to_string()).context("Could not get arg.")?.as_channel().context("Could not convert arg to Channel.")
    }

    pub fn get_role_arg<T: ToString>(&self, arg: T) -> Result<&Role> {
        self.input.args.get(&arg.to_string()).context("Could not get arg.")?.as_role().context("Could not convert arg to Role.")
    }

    pub fn get_f64_arg<T: ToString>(&self, arg: T) -> Result<f64> {
        self.input.args.get(&arg.to_string()).context("Could not get arg.")?.as_f64().context("Could not convert arg to f64.")
    }

    pub fn get_attachment_arg<T: ToString>(&self, arg: T) -> Result<&Attachment> {
        self.input.args.get(&arg.to_string()).context("Could not get arg.")?.as_attachment().context("Could not convert arg to Attachment.")
    }
}
