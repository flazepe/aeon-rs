use crate::{
    macros::if_else,
    statics::{
        emojis::{ERROR_EMOJI, SUCCESS_EMOJI},
        CACHE,
    },
};
use anyhow::{bail, Result};
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
use std::{
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct Interaction<'a> {
    input: &'a CommandInput,
    res: &'a CommandResponder,
}

impl<'a> Interaction<'a> {
    pub fn new(input: &'a CommandInput, res: &'a CommandResponder) -> Interaction<'a> {
        Self { input, res }
    }

    pub async fn verify(self) -> Result<Interaction<'a>> {
        if CACHE.cooldowns.read().unwrap().get(&self.input.user.id).unwrap_or(&0)
            > &SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
        {
            self.respond_error("You are under a cooldown. Try again later.", true)
                .await?;

            bail!("User is under a cooldown.");
        }

        if let Some(interaction) = self
            .input
            .message
            .as_ref()
            .and_then(|message| message.interaction.as_ref())
        {
            if self.input.user.id != interaction.user.id {
                self.respond_error("This isn't your interaction.", true).await?;
                bail!("User is not the interaction initiator.");
            }
        }

        Ok(self)
    }

    pub async fn respond<T: Into<MessageResponse>>(&self, response: T, ephemeral: bool) -> Result<()> {
        CACHE.cooldowns.write().unwrap().insert(
            self.input.user.id.clone(),
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3,
        );

        let response = response.into().set_ephemeral(ephemeral);

        if_else!(
            self.input.message.is_some(),
            self.res.update_message(response).await?,
            self.res.send_message(response).await?,
        );

        Ok(())
    }

    pub async fn respond_error<T: Display>(&self, response: T, ephemeral: bool) -> Result<()> {
        self.respond(format!("{ERROR_EMOJI} {response}"), ephemeral).await
    }

    pub async fn respond_success<T: Display>(&self, response: T, ephemeral: bool) -> Result<()> {
        self.respond(format!("{SUCCESS_EMOJI} {response}"), ephemeral).await
    }
}
