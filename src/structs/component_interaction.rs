use crate::{
    macros::if_else,
    statics::{emojis::ERROR_EMOJI, CACHE},
};
use anyhow::{bail, Result};
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ComponentInteraction<'a> {
    input: &'a CommandInput,
    res: &'a CommandResponder,
}

impl<'a> ComponentInteraction<'a> {
    pub async fn verify(input: &'a CommandInput, res: &'a CommandResponder) -> Result<ComponentInteraction<'a>> {
        if CACHE.cooldowns.read().unwrap().get(&input.user.id).unwrap_or(&0)
            > &SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
        {
            res.send_message(
                MessageResponse::from(format!("{ERROR_EMOJI} You are under a cooldown. Try again later."))
                    .set_ephemeral(true),
            )
            .await?;

            bail!("User is under a cooldown.");
        }

        if let Some(interaction) = input.message.as_ref().and_then(|message| message.interaction.as_ref()) {
            if input.user.id != interaction.user.id {
                res.send_message(
                    MessageResponse::from(format!("{ERROR_EMOJI} This isn't your interaction.")).set_ephemeral(true),
                )
                .await?;

                bail!("User is not the interaction initiator.");
            }
        }

        Ok(Self { input, res })
    }

    pub async fn respond<T: Into<MessageResponse>>(self, response: T, ephemeral: bool) -> Result<()> {
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
}
