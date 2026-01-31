use crate::statics::{CONFIG, REST};
use anyhow::Result;
use base64::{Engine, prelude::BASE64_STANDARD};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_str, json};
use slashook::structs::Emoji as SlashookEmoji;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::{read, read_dir, read_to_string, write},
};
use tracing::info;

const LAST_UPDATED_TIMESTAMP: u32 = 1760020150;

#[derive(Default, Debug)]
pub struct EmojiManager {
    emojis: HashMap<String, Emoji>,
    last_updated_timestamp: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Emoji {
    name: String,
    id: String,
    animated: bool,
}

#[derive(Deserialize, Debug)]
struct Emojis {
    emojis: Vec<Emoji>,
    last_updated_timestamp: u32,
}

#[derive(Deserialize, Debug)]
struct APIApplicationEmojis {
    items: Vec<Emoji>,
}

impl EmojiManager {
    pub fn new() -> Self {
        let Ok(emojis_file) = read_to_string("emojis.json") else { return Default::default() };
        let Ok(emojis) = from_str::<Emojis>(&emojis_file) else { return Default::default() };

        Self {
            emojis: HashMap::from_iter(emojis.emojis.into_iter().map(|emoji| (emoji.name.clone(), emoji))),
            last_updated_timestamp: emojis.last_updated_timestamp,
        }
    }

    pub async fn sync(&mut self) -> Result<()> {
        if self.last_updated_timestamp == LAST_UPDATED_TIMESTAMP {
            info!(target: "Emojis", "Already synced. Skipping sync.");
            return Ok(());
        }

        info!(target: "Emojis", "Not synced. Syncing...");

        self.emojis.clear();

        let emojis = REST
            .get::<APIApplicationEmojis>(format!("applications/{}/emojis", CONFIG.bot.client_id))
            .await
            .map(|emojis| emojis.items)?
            .into_iter()
            .filter(|emoji| emoji.name.starts_with("aeon_"));

        for emoji in emojis {
            REST.delete::<()>(format!("applications/{}/emojis/{}", CONFIG.bot.client_id, emoji.id)).await?;
        }

        for entry in read_dir("emojis")? {
            let entry = entry?;
            let filename = entry.file_name();

            if !filename.to_string_lossy().starts_with("aeon_") {
                continue;
            }

            let base64 = BASE64_STANDARD.encode(read(entry.path())?);
            let body = json!({
                "name": filename.to_string_lossy().split(".").next().unwrap_or_default(),
                "image": format!("data:image/png;base64,{base64}"),
            });

            let emoji = REST.post::<Emoji, Value>(format!("applications/{}/emojis", CONFIG.bot.client_id), body).await?;
            self.emojis.insert(emoji.name.clone(), emoji);
        }

        write(
            "emojis.json",
            format!(
                "{}\n",
                json!({
                    "emojis": self.emojis.values().collect::<Vec<&Emoji>>(),
                    "last_updated_timestamp": LAST_UPDATED_TIMESTAMP,
                }),
            ),
        )?;

        Ok(())
    }

    pub fn get<T: Display, U: Display>(&self, name: T, unicode_fallback: U) -> SlashookEmoji {
        if let Some(emoji) = self.emojis.get(&name.to_string()) {
            SlashookEmoji::new_custom_emoji(&emoji.id, &emoji.name, emoji.animated)
        } else {
            SlashookEmoji::new_standard_emoji(unicode_fallback)
        }
    }
}
