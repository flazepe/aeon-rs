use crate::statics::REQWEST;
use anyhow::Result;
use slashook::{
    commands::{CommandResponder, MessageResponse},
    structs::utils::File,
};
use std::{fmt::Display, process::Stdio};
use tokio::{io::AsyncWriteExt, process::Command};

pub struct VoiceMessage;

impl VoiceMessage {
    pub async fn send<T: Display>(res: &CommandResponder, audio_url: T, ephemeral: bool) -> Result<()> {
        res.send_message(MessageResponse::from("Sending voice message...").set_ephemeral(ephemeral)).await?;

        let mut child = Command::new("ffprobe")
            .args(["-i", "-", "-show_entries", "format=duration", "-v", "quiet", "-of", "csv=p=0"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let audio_bytes = REQWEST.get(audio_url.to_string()).send().await?.bytes().await?;
        child.stdin.take().unwrap().write_all(&audio_bytes).await?;

        if res
            .send_followup_message(
                MessageResponse::from(
                    File::new("voice-message.ogg", audio_bytes)
                        .set_duration_secs(String::from_utf8(child.wait_with_output().await?.stdout)?.trim().parse::<f64>().unwrap_or(0.))
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
