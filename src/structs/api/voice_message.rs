use crate::statics::REQWEST;
use anyhow::Result;
use futures::TryFutureExt;
use slashook::{
    commands::{CommandResponder, MessageResponse},
    structs::utils::File,
};
use std::{fmt::Display, process::Stdio, sync::Arc};
use tokio::{io::AsyncWriteExt, process::Command, spawn};

pub struct VoiceMessage;

impl VoiceMessage {
    pub async fn send<T: Display>(res: &CommandResponder, audio_url: T, ephemeral: bool) -> Result<()> {
        res.send_message(MessageResponse::from("Sending voice message...").set_ephemeral(ephemeral)).await?;

        let Ok(bytes) = REQWEST.get(audio_url.to_string()).send().and_then(|res| res.bytes()).await.map(Arc::new) else {
            res.edit_original_message("Please provide a valid audio URL.").await?;
            return Ok(());
        };

        let mut child = Command::new("ffprobe")
            .args(["-i", "-", "-show_entries", "packet=dts_time", "-of", "csv=p=0", "-v", "quiet"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = child.stdin.take().unwrap();
        let task_bytes = bytes.clone();
        let handle = spawn(async move { stdin.write_all(&task_bytes).await.unwrap() });
        let duration_secs = String::from_utf8(child.wait_with_output().await?.stdout)?
            .trim()
            .split('\n')
            .last()
            .map_or(0., |line| line.replace(',', "").parse::<f64>().unwrap_or(0.));
        handle.await?; // Wait until task is done before getting Arc's inner value

        if let Err(error) = res
            .send_followup_message(
                MessageResponse::from(
                    File::new("voice-message.ogg", Arc::into_inner(bytes).unwrap()).set_duration_secs(duration_secs).set_waveform(""),
                )
                .set_ephemeral(ephemeral)
                .set_as_voice_message(true),
            )
            .await
        {
            res.edit_original_message(format!(
                "`{error}`\nMake sure the file is a valid audio file and I have the permission to send voice messages.",
            ))
            .await?;
        }

        Ok(())
    }
}
