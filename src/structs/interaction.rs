use crate::statics::{
    emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
    CACHE,
};
use anyhow::{bail, Context, Result};
use slashook::{
    commands::{CommandInput, CommandResponder, MessageResponse},
    structs::interactions::ApplicationCommandOptionChoice,
};
use std::collections::hash_map::Iter;
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct Interaction<'a> {
    input: &'a CommandInput,
    res: &'a CommandResponder,
    verified: bool,
}

impl<'a> Interaction<'a> {
    pub fn new(input: &'a CommandInput, res: &'a CommandResponder) -> Interaction<'a> {
        Self { input, res, verified: false }
    }

    pub async fn verify(mut self) -> Result<Interaction<'a>> {
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

        CACHE.cooldowns.write().unwrap().insert(self.input.user.id.clone(), SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3);

        let response = response.into().set_ephemeral(ephemeral);

        match self.input.message.is_some() && !ephemeral {
            true => self.res.update_message(response).await?,
            false => self.res.send_message(response).await?,
        };

        Ok(())
    }

    pub async fn respond_error<T: Display>(&self, response: T, ephemeral: bool) -> Result<()> {
        self.respond(format!("{ERROR_EMOJI} {response}"), ephemeral).await
    }

    pub async fn respond_success<T: Display>(&self, response: T, ephemeral: bool) -> Result<()> {
        self.respond(format!("{SUCCESS_EMOJI} {response}"), ephemeral).await
    }

    pub async fn hashmap_autocomplete<K: ToString, V: ToString>(&self, hashmap_iter: Iter<'_, K, V>) -> Result<()> {
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
                hashmap_iter
                    .filter(|(k, v)| k.to_string().to_lowercase().contains(&value) || v.to_string().to_lowercase().contains(&value))
                    .map(|(k, v)| ApplicationCommandOptionChoice::new(v.to_string(), k.to_string()))
                    .take(25)
                    .collect(),
            )
            .await?)
    }
}
