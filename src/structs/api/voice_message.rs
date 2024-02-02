use crate::statics::{REQWEST, REST};
use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::json;
use slashook::structs::channels::Message;
use std::fmt::Display;

pub struct VoiceMessage;

impl VoiceMessage {
    pub async fn send<T: Display, U: ToString, V: ToString>(channel_id: T, message_id: Option<U>, audio_url: V) -> Result<Message> {
        let bytes = REQWEST.get(audio_url.to_string()).send().await?.bytes().await?;

        let attachments = REST
            .post::<ChannelAttachments, _>(
                format!("channels/{channel_id}/attachments"),
                json!({
                    "files": [{ "id": "42069", "filename": "voice-message.ogg", "file_size": bytes.len() }],
                }),
            )
            .await?;

        REQWEST.put(&attachments.attachments[0].upload_url).body(bytes).send().await?;

        match REST
            .post::<Message, _>(
                format!("channels/{channel_id}/messages"),
                json!({
                    "attachments": [{
                       "id": "42069",
                       "filename": "voice-message.ogg",
                       "uploaded_filename": attachments.attachments[0].upload_filename,
                       "duration_secs": 0,
                       "waveform": "",
                   }],
                   "flags": 1 << 13,
                   "message_reference": match message_id.as_ref() {
                        Some(message_id) => json!({ "message_id": message_id.to_string() }),
                        None => json!(null),
                    },
                   "allowed_mentions": { "replied_user": false },
                }),
            )
            .await
        {
            Ok(message) => Ok(message),
            Err(_) => bail!(
                "Could not send voice message. Make sure the file is a valid audio file and I have the permission to send voice messages.",
            ),
        }
    }
}

#[derive(Deserialize)]
struct ChannelAttachments {
    attachments: Vec<ChannelAttachment>,
}

#[derive(Deserialize)]
struct ChannelAttachment {
    upload_url: String,
    upload_filename: String,
}
