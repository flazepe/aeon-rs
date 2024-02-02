use crate::statics::REQWEST;
use anyhow::Result;
use slashook::{
    commands::{CommandResponder, MessageResponse},
    structs::utils::File,
};

pub struct VoiceMessage;

impl VoiceMessage {
    pub async fn send<T: ToString>(res: &CommandResponder, audio_url: T, ephemeral: bool) -> Result<()> {
        res.send_message(MessageResponse::from("Sending voice message...").set_ephemeral(ephemeral)).await?;

        if res
            .send_followup_message(
                MessageResponse::from(
                    File::new("voice-message.ogg", REQWEST.get(audio_url.to_string()).send().await?.bytes().await?)
                        .set_duration_secs(0.)
                        .set_waveform(""),
                )
                .set_ephemeral(ephemeral)
                .set_as_voice_message(true),
            )
            .await
            .is_err()
        {
            res.edit_original_message(
                "Could not send voice message. Make sure the file is a valid audio file and I have the permission to send voice messages.",
            )
            .await?;
        }

        Ok(())
    }
}
