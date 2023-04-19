use crate::macros::if_else;
use anyhow::{bail, Result};
use slashook::commands::{CommandInput, CommandResponder, MessageResponse};

pub struct ComponentInteraction<'a> {
    input: &'a CommandInput,
    res: &'a CommandResponder,
}

impl<'a> ComponentInteraction<'a> {
    pub async fn verify(input: &'a CommandInput, res: &'a CommandResponder) -> Result<ComponentInteraction<'a>> {
        if let Some(interaction) = input.message.as_ref().and_then(|message| message.interaction.as_ref()) {
            if input.user.id != interaction.user.id {
                res.send_message(
                    MessageResponse::from(format!(
                        "{} This isn't your interaction.",
                        crate::statics::emojis::ERROR_EMOJI
                    ))
                    .set_ephemeral(true),
                )
                .await?;

                bail!("User is not the interaction initiator.");
            }
        }

        Ok(Self { input, res })
    }

    pub async fn respond<T: Into<MessageResponse>>(self, response: T) -> Result<()> {
        if_else!(
            self.input.message.is_some(),
            self.res.update_message(response).await?,
            self.res.send_message(response).await?,
        );

        Ok(())
    }
}
