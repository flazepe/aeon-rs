use crate::statics::REQWEST;
use anyhow::Result;
use base64::{prelude::BASE64_STANDARD, Engine};
use futures::TryFutureExt;
use prost::bytes::Bytes;
use reqwest::Result as ReqwestResult;
use slashook::{
    commands::{CommandResponder, MessageResponse},
    structs::utils::File,
};
use std::{ffi::OsStr, fmt::Display, process::Stdio};
use tokio::{io::AsyncWriteExt, process::Command, spawn};

pub struct VoiceMessage<'a> {
    res: &'a CommandResponder,
    audio_url: String,
    bytes: ReqwestResult<Bytes>,
    ephemeral: bool,
}

impl<'a> VoiceMessage<'a> {
    pub async fn new<T: Display>(res: &'a CommandResponder, audio_url: T, ephemeral: bool) -> Result<Self> {
        res.send_message(MessageResponse::from(format!("Sending {audio_url} as voice message...")).set_ephemeral(ephemeral)).await?;

        Ok(Self {
            res,
            audio_url: audio_url.to_string(),
            bytes: REQWEST.get(audio_url.to_string()).send().and_then(|res| res.bytes()).await,
            ephemeral,
        })
    }

    pub async fn send(self) -> Result<()> {
        let duration_secs = self.get_duration_secs().await?;
        let waveform = self.get_waveform(duration_secs).await?;

        let Ok(bytes) = self.bytes else {
            self.res.edit_original_message("Please provide a valid media URL.").await?;
            return Ok(());
        };

        self.res
            .edit_original_message(
                match self
                    .res
                    .send_followup_message(
                        MessageResponse::from(
                            File::new("voice-message.ogg", bytes.clone()).set_duration_secs(duration_secs).set_waveform(waveform),
                        )
                        .set_ephemeral(self.ephemeral)
                        .set_as_voice_message(true),
                    )
                    .await
                {
                    Ok(_) => format!("Sent {} as voice message.", self.audio_url),
                    Err(error) => {
                        format!(
                            "Could not {} as voice message. Make sure the file is a valid media file with audio and I have the Send Voice Messages permission.\n`{error}`",
                            self.audio_url,
                        )
                    },
                },
            )
            .await?;

        Ok(())
    }

    async fn run_command<T: AsRef<OsStr>, U: IntoIterator<Item = V>, V: AsRef<OsStr>>(&self, command: T, args: U) -> Result<Vec<u8>> {
        let Ok(bytes) = self.bytes.as_ref() else { return Ok(vec![]) };

        let mut child = Command::new(command).args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
        let mut stdin = child.stdin.take().unwrap();
        let task_bytes = bytes.clone();
        let handle = spawn(async move { stdin.write_all(&task_bytes).await.unwrap() });
        let output = child.wait_with_output().await?;
        handle.await?; // Wait until task is done to be able to get Arc's inner value if needed

        Ok(output.stdout)
    }

    async fn get_duration_secs(&self) -> Result<f64> {
        Ok(String::from_utf8_lossy(&self.run_command("ffprobe", ["-i", "-", "-show_entries", "packet=dts_time", "-of", "csv=p=0"]).await?)
            .trim()
            .split('\n')
            .last()
            .map_or(0., |line| line.replace(',', "").parse::<f64>().unwrap_or(0.)))
    }

    async fn get_waveform(&self, duration_secs: f64) -> Result<String> {
        Ok(BASE64_STANDARD
            .encode(
                self.run_command(
                    "ffmpeg",
                    [
                        "-i",
                        "-",
                        "-vn",
                        "-map",
                        "0:a",
                        "-ac",
                        "1",
                        "-af",
                        &format!("atempo={}", duration_secs / 25.6),
                        "-c:a",
                        "pcm_s8",
                        "-f",
                        "data",
                        "-",
                    ],
                    // ["-i", "-", "-vn", "-map", "0:a", "-ac", "1", "-af", "aresample=1", "-c:a", "pcm_s16le", "-f", "data", "-"],
                )
                .await?,
            )
            .chars()
            .take(400)
            .collect())
    }
}
